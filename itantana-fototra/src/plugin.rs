use std::{
    any::Any,
    ffi::{CStr, CString, c_char, c_int, c_void},
    panic::catch_unwind,
    str::FromStr,
    sync::Arc,
};

use crate::{c_registry::CRegistry, registry::Registry};

#[repr(C)]
pub struct Plugin {
    pub name: *const c_char,
    pub version: *const c_char,
    pub prepare_fn: extern "C" fn(
        *mut CRegistry,
        extern "C" fn(
            *mut CRegistry,
            *const c_char,
            *const c_void,
            unsafe extern "C" fn(*mut c_void),
        ) -> c_int,
    ),
    pub initialize_fn: extern "C" fn(
        *const CRegistry,
        extern "C" fn(*const CRegistry, *const c_char, *mut *const c_void),
    ),
}

unsafe impl Sync for Plugin {}
unsafe impl Send for Plugin {}

impl Plugin {
    pub fn new(
        name: &str,
        version: &str,
        prepare_fn: extern "C" fn(
            *mut CRegistry,
            extern "C" fn(
                *mut CRegistry,
                *const c_char,
                *const c_void,
                unsafe extern "C" fn(*mut c_void),
            ) -> c_int,
        ),
        initialize_fn: extern "C" fn(
            *const CRegistry,
            extern "C" fn(*const CRegistry, *const c_char, *mut *const c_void),
        ),
    ) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Plugin name cannot be empty".to_string());
        }
        if version.is_empty() {
            return Err("Plugin version cannot be empty".to_string());
        }
        let name = CString::new(name).unwrap();
        let version = CString::new(version).unwrap();
        Ok(Plugin {
            name: name.as_ptr(),
            version: version.as_ptr(),
            prepare_fn,
            initialize_fn,
        })
    }

    pub fn get_name(&self) -> String {
        let c_name = unsafe { CStr::from_ptr(self.name) };
        String::from_str(c_name.to_str().unwrap()).unwrap()
    }

    pub fn prepare(
        &self,
        registry: Arc<Registry>,
        registry_register_fn: extern "C" fn(
            *mut CRegistry,
            *const c_char,
            *const c_void,
            unsafe extern "C" fn(*mut c_void),
        ) -> c_int,
    ) -> Result<(), String> {
        let mut guard = registry.inner.write().unwrap();
        let c_registry = &raw mut *guard;

        //        unsafe { drop_in_place(c_registry) };
        //        drop(guard);
        catch_unwind(|| (self.prepare_fn)(c_registry, registry_register_fn))
            .map_err(extract_panic_message)
    }

    pub fn initialize(
        &self,
        registry: Arc<Registry>,
        registry_get_fn: extern "C" fn(*const CRegistry, *const c_char, *mut *const c_void),
    ) -> Result<(), String> {
        let mut guard = registry.inner.write().unwrap();
        let c_registry = &raw mut *guard;

        catch_unwind(|| (self.initialize_fn)(c_registry, registry_get_fn))
            .map_err(extract_panic_message)
    }
}

fn extract_panic_message(err: Box<dyn Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic".to_string()
    }
}
