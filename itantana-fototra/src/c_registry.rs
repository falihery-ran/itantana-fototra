use std::{
    collections::HashMap,
    ffi::{CStr, CString, c_char, c_int, c_void},
    ptr::null,
    str::FromStr,
};

#[derive(Debug)]
struct Entry {
    pub ptr: *const c_void,
    pub drop_fn: unsafe extern "C" fn(*mut c_void),
}

impl Entry {
    pub fn new(
        ptr: *const c_void,
        drop_fn: unsafe extern "C" fn(*mut c_void),
    ) -> Result<Self, String> {
        if ptr.is_null() {
            Err(String::from_str("Cannot create Entry from null ptr").unwrap())
        } else {
            Ok(Self { ptr, drop_fn })
        }
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        unsafe { (self.drop_fn)(self.ptr as *mut c_void) }
    }
}

#[repr(C)]
pub struct CRegistry {
    pub _private: *mut c_void,
}

impl Drop for CRegistry {
    fn drop(&mut self) {
        if !self._private.is_null() {
            let _ = unsafe { Box::from_raw(self._private as *mut HashMap<String, Entry>) };
        }
    }
}

unsafe impl Sync for CRegistry {}
unsafe impl Send for CRegistry {}

//static mut C_RUNTIME: *mut CRuntime = null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn c_registry_new() -> *mut CRegistry {
    let hashmap: HashMap<String, Entry> = HashMap::new();
    let box_hashmap = Box::new(hashmap);
    let c_runtime = Box::new(CRegistry {
        _private: Box::into_raw(box_hashmap) as *mut c_void,
    });
    Box::into_raw(c_runtime)
}

// #[unsafe(no_mangle)]
// pub extern "C" fn get_c_runtime() -> *mut CRuntime {
//     if unsafe { C_RUNTIME.is_null() } {
//         unsafe { C_RUNTIME = c_runtime_new() }
//     }
//     unsafe { C_RUNTIME }
// }

// #[unsafe(no_mangle)]
// pub extern "C" fn free_c_runtime() {
//     if !unsafe { C_RUNTIME.is_null() } {
//         unsafe { drop_in_place(C_RUNTIME) };
//         unsafe { C_RUNTIME = null_mut() }
//     }
// }

#[unsafe(no_mangle)]
pub extern "C" fn c_registry_register(
    c_registry: *mut CRegistry,
    name: *const c_char,
    object: *const c_void,
    object_free_fn: unsafe extern "C" fn(*mut c_void),
    err_msg: *mut *const c_char,
) -> c_int {
    //    unsafe { drop_in_place(*err_msg as *mut c_char) };
    // Reset error
    // unsafe { *err_msg = null_mut() };
    unsafe {
        if !err_msg.is_null() && !(*err_msg).is_null() {
            let _ = CString::from_raw(*err_msg as *mut c_char);
            *err_msg = null();
        }
    }
    if c_registry.is_null() {
        return set_err(err_msg, "c_registry cannot be null");
    }

    if name.is_null() {
        return set_err(err_msg, "name cannot be null");
    }

    let c_name = unsafe { CStr::from_ptr(name) };
    let string_name = match c_name.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return set_err(err_msg, "name is not valid UTF-8"),
    };

    let entry = match Entry::new(object, object_free_fn) {
        Ok(e) => e,
        Err(e) => return set_err(err_msg, &e),
    };

    //let mut boxed_hasmap = unsafe { Box::from_raw(runtime._private as *mut HashMap<String, Entry>) };

    unsafe {
        if ((*c_registry)._private).is_null() {
            return set_err(err_msg, "c_registry._private cannot be null");
        }
    }

    let _ = unsafe {
        (*((*c_registry)._private as *mut HashMap<String, Entry>)).insert(string_name, entry)
    };

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn c_registry_get(
    c_registry: *const CRegistry,
    name: *const c_char,
    c_object: *mut *const c_void,
    err_msg: *mut *const c_char,
) {
    //    unsafe { drop_in_place(*err_msg as *mut c_char) };
    // Reset error
    unsafe {
        if !err_msg.is_null() && !(*err_msg).is_null() {
            let _ = CString::from_raw(*err_msg as *mut c_char);
            *err_msg = null();
        }
    }
    //    unsafe { *err_msg = null_mut() };
    if c_registry.is_null() {
        set_err(err_msg, "c_runtime cannot be null");
        unsafe { *c_object = null() };
    }

    let c_name = unsafe { CStr::from_ptr(name) };
    let string_name = match c_name.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => {
            set_err(err_msg, "name is not valid UTF-8");
            unsafe { *c_object = null() };
            return;
        }
    };

    unsafe {
        if ((*c_registry)._private).is_null() {
            set_err(err_msg, "c_registry._private cannot be null");
            *c_object = null();
        }

        let ret = (*((*c_registry)._private as *const HashMap<String, Entry>))
            .get(&string_name)
            .map_or(null(), |entry| entry.ptr as *const c_void);
        *c_object = ret;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn c_registry_free(c_registry: *mut CRegistry) {
    let _ = unsafe { Box::from_raw(c_registry) };
}

// Helper to set error message (caller must free via c_runtime_free_error)
fn set_err(err_msg: *mut *const c_char, msg: &str) -> c_int {
    let cstring = match CString::new(msg) {
        Ok(c) => c,
        Err(_) => return -1,
    };
    let ptr = cstring.into_raw();
    unsafe { *err_msg = ptr as *const c_char };
    -1
}
