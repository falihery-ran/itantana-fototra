use std::{
    collections::HashMap,
    env,
    ffi::{CStr, CString, c_char, c_int},
    mem::zeroed,
    os::raw::c_void,
    path::PathBuf,
    ptr::null_mut,
    str::FromStr,
    sync::{Arc, Mutex, OnceLock, RwLock},
};

use futures::future::join_all;
use libloading::{Library, Symbol};

use crate::{
    Plugin,
    c_registry::{CRegistry, c_registry_get, c_registry_new, c_registry_register},
    configuration::{Configuration, load_configuration},
};

// Store the lib inside static var so that it will not dropped
// because when it is drop, all extern fn dynamically loaded become invalid and we get seg fault at the end
static LIBLOADING: OnceLock<Library> = OnceLock::new();

static PLUGIN_LIST: OnceLock<Arc<Mutex<Vec<Arc<Plugin>>>>> = OnceLock::new();

static RUNTIME_INSTANCE: OnceLock<Arc<Registry>> = OnceLock::new();

#[derive(Clone)]
pub struct Registry {
    pub inner: Arc<RwLock<CRegistry>>,
}

impl Registry {
    pub fn new() -> Arc<Self> {
        let c_registry_ptr = c_registry_new();
        let c_registry = unsafe { *Box::from_raw(c_registry_ptr) };
        Arc::new(Self {
            inner: Arc::new(RwLock::new(c_registry)),
        })
    }

    pub fn try_from_c_registry(c_registry: *mut CRegistry) -> Result<Arc<Self>, String> {
        if c_registry.is_null() {
            Err(String::from_str("Cannot create Registry from null ptr").unwrap())
        } else {
            let c_registry = unsafe { Box::from_raw(c_registry) };
            Ok(Arc::new(Self {
                inner: Arc::new(RwLock::new(*c_registry)),
            }))
        }
    }

    pub async fn init(self: Arc<Self>) -> Result<(), String> {
        let configuration = load_configuration();
        self.clone()
            .register("configuration", Arc::new(configuration.clone()))?;
        let conf_plugin_list: HashMap<String, Configuration> = configuration
            .get("plugins")
            .ok_or("cannot get plugins from the configuration")?
            .clone()
            .try_into()?;

        let mut plugin_list = PLUGIN_LIST
            .get_or_init(|| Arc::new(Mutex::new(Vec::new())))
            .lock()
            .unwrap();

        for (plugin_name_in_conf, plugin_conf) in conf_plugin_list {
            let mut path = TryInto::<String>::try_into(
                plugin_conf
                    .get("path")
                    .cloned()
                    .ok_or_else(|| format!("plugin {} path is undefined", plugin_name_in_conf))?,
            )?;
            if !PathBuf::from_str(&path).unwrap().exists() {
                path = env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(path)
                    .to_str()
                    .unwrap()
                    .to_string();
            }
            let mut err_msg = "Failed to load".to_string();
            err_msg.push_str(path.as_str());
            let lib = unsafe { Library::new(path).expect(err_msg.as_str()) };
            LIBLOADING.set(lib).unwrap();
            let lib = LIBLOADING.get().unwrap();
            let get_plugin_name: Symbol<extern "C" fn(*mut *const c_char)> =
                unsafe { lib.get(b"get_plugin_name").unwrap() };
            let get_plugin_version: Symbol<extern "C" fn(*mut *const c_char)> =
                unsafe { lib.get(b"get_plugin_version").unwrap() };
            let plugin_prepare: Symbol<
                extern "C" fn(
                    *mut CRegistry,
                    extern "C" fn(
                        *mut CRegistry,
                        *const c_char,
                        *const c_void,
                        unsafe extern "C" fn(*mut c_void),
                    ) -> c_int,
                ),
            > = unsafe { lib.get(b"plugin_prepare").unwrap() };
            let plugin_initiliaze: Symbol<
                extern "C" fn(
                    *const CRegistry,
                    extern "C" fn(*const CRegistry, *const c_char, *mut *const c_void),
                ),
            > = unsafe { lib.get(b"plugin_initialize").unwrap() };

            let mut const_name_ptr: *const c_char = null_mut();
            get_plugin_name(&mut const_name_ptr);
            let name = unsafe { CString::from_raw(const_name_ptr as *mut c_char) };

            let mut const_version_ptr: *const c_char = null_mut();
            get_plugin_version(&mut const_version_ptr);
            let version = unsafe { CString::from_raw(const_version_ptr as *mut c_char) };

            let plugin = Plugin::new(
                name.to_str()
                    .map_err(|e| format!("Cannot convert name to String: {}", e))?,
                version
                    .to_str()
                    .map_err(|e| format!("Cannot convert version to String: {}", e))?,
                *plugin_prepare,
                *plugin_initiliaze,
            )?;

            let registry = self.clone();
            plugin.prepare(registry, prepare).unwrap();
            plugin_list.push(Arc::new(plugin));
        }

        let mut task_array = Vec::new();

        for plugin in &*plugin_list {
            let registry = self.clone();
            let plugin = plugin.clone();
            task_array.push(tokio::spawn(async move {
                plugin.initialize(registry, initialize)
            }));
        }
        let _ = join_all(task_array).await;
        Ok(())
    }

    pub fn register<T>(self: Arc<Self>, key: &str, object: Arc<T>) -> Result<(), String> {
        let name = CString::from_str(key).map_err(|e| e.to_string())?;
        let mut err_msg = unsafe { zeroed::<*const c_char>() };

        let mut guard = self.inner.write().unwrap();
        let c_registry = &raw mut *guard;

        let object = object.clone();

        if c_registry_register(
            c_registry,
            name.as_ptr(),
            Arc::into_raw(object) as *const c_void,
            drop_arc::<T>,
            &raw mut err_msg,
        ) < 0
        {
            let msg = unsafe { CStr::from_ptr(err_msg) };
            let msg = msg.to_str().map_err(|e| e.to_string())?;
            return Err(msg.to_string());
        }
        Ok(())
    }

    pub fn get<T>(self: Arc<Self>, key: &str) -> Result<Option<Arc<T>>, String> {
        let name = CString::from_str(key).map_err(|e| e.to_string())?;
        let mut err_msg = unsafe { zeroed::<*const c_char>() };
        let registry = self.clone();
        let guard = registry.inner.read().unwrap();
        let c_registry = &*guard;

        let mut c_object: *const c_void = unsafe { zeroed() };
        c_registry_get(
            c_registry,
            name.as_ptr(),
            &raw mut c_object,
            &raw mut err_msg,
        );
        if c_object.is_null() {
            if err_msg.is_null() {
                Ok(None)
            } else {
                let msg = unsafe { CStr::from_ptr(err_msg) };
                let msg = msg.to_str().map_err(|e| e.to_string())?;
                Err(msg.to_string())
            }
        } else {
            // CRITICAL: Don't take ownership - increment the refcount instead
            let arc = unsafe { Arc::from_raw(c_object as *const T) };
            let cloned = arc.clone();
            // Put the Arc back without dropping it
            let _ = Arc::into_raw(arc);
            Ok(Some(cloned))
        }
    }

    pub fn get_instance() -> Arc<Self> {
        RUNTIME_INSTANCE.get().unwrap().clone()
    }
    pub fn persist(self: Arc<Self>) {
        let _ = RUNTIME_INSTANCE.set(self.clone());
    }
}

unsafe extern "C" fn drop_arc<T>(ptr: *mut c_void) {
    let _ = unsafe { Arc::from_raw(ptr as *const T) };
}

extern "C" fn prepare(
    c_registry: *mut CRegistry,
    c_name: *const c_char,
    c_object: *const c_void,
    c_drop_fn: unsafe extern "C" fn(*mut c_void),
) -> c_int {
    if c_registry.is_null() {
        panic!("prepare: c_registry cannot be ptr null");
    }
    if c_name.is_null() {
        panic!("prepare: c_name cannot be ptr null");
    }
    if c_object.is_null() {
        panic!("prepare: c_object cannot be ptr null");
    }
    let mut err_msg = unsafe { zeroed::<*const c_char>() };
    let ret = c_registry_register(c_registry, c_name, c_object, c_drop_fn, &raw mut err_msg);
    unsafe {
        if !err_msg.is_null() {
            let _ = CString::from_raw(err_msg as *mut c_char);
        }
    }
    ret
}

extern "C" fn initialize(
    c_registry: *const CRegistry,
    c_name: *const c_char,
    c_object: *mut *const c_void,
) {
    if c_registry.is_null() {
        panic!("prepare: c_registry cannot be ptr null");
    }
    if c_name.is_null() {
        panic!("prepare: c_name cannot be ptr null");
    }
    let mut err_msg = unsafe { zeroed::<*const c_char>() };
    c_registry_get(c_registry, c_name, c_object, &raw mut err_msg);
}
