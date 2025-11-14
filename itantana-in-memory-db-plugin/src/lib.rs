pub mod permission_repository;
pub mod user_internet_repository;
pub mod user_password_policy_repository;
pub mod user_password_repository;
pub mod user_permission_repository;
pub mod user_repository;

use fototra::{
    c_registry::CRegistry,
    repository::{
        permission_repository::PermissionRepository,
        user_internet_repository::UserInternetRepository,
        user_password_policy_repository::UserPasswordPolicyRepository,
        user_password_repository::UserPasswordRepository,
        user_permission_repository::UserPermissionRepository, user_repository::UserRepository,
    },
};
use std::{
    ffi::{CString, c_char, c_int},
    mem::zeroed,
    os::raw::c_void,
    str::FromStr,
    sync::Arc,
};

use crate::{
    permission_repository::InMemoryPermissionRepository,
    user_internet_repository::InMemoryUserInternetRepository,
    user_password_policy_repository::InMemoryUserPasswordPolicyRepository,
    user_password_repository::InMemoryUserPasswordRepository,
    user_permission_repository::InMemoryUserPermissionRepository,
    user_repository::InMemoryUserRepository,
};

// #[unsafe(no_mangle)]
// pub fn register_plugin() -> Plugin {
//     Plugin { name: env!("CARGO_CRATE_NAME").to_string(), version: env!("CARGO_PKG_VERSION").to_string(),
//     prepare_fn: Some(prepare()),
//     initialize_fn: Some(initialize())}
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_plugin_name(name: *mut *const c_char) {
    let c_string = CString::new(env!("CARGO_CRATE_NAME")).unwrap();
    unsafe { *name = c_string.into_raw() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_plugin_version(version: *mut *const c_char) {
    let c_string = CString::new(env!("CARGO_PKG_VERSION")).unwrap();
    unsafe { *version = c_string.into_raw() };
}

pub unsafe extern "C" fn dropper<T>(ptr: *mut c_void) {
    let _ = unsafe { Arc::from_raw(ptr as *const T) };
}

#[unsafe(no_mangle)]
pub extern "C" fn plugin_prepare(
    c_registry: *mut CRegistry,
    registry_register_fn: extern "C" fn(
        *mut CRegistry,
        *const c_char,
        *const c_void,
        unsafe extern "C" fn(*mut c_void),
    ) -> c_int,
) {
    let c_name = CString::from_str("permission_repository").unwrap();
    if registry_register_fn(
        c_registry,
        c_name.as_ptr(),
        Arc::into_raw(Arc::new(PermissionRepository::new(Arc::new(
            InMemoryPermissionRepository::new(),
        )))) as *const c_void as *mut c_void,
        dropper::<PermissionRepository>,
    ) < 0
    {
        panic!("Cannot register {}", c_name.to_str().unwrap());
    }

    let c_name = CString::from_str("user_repository").unwrap();
    if registry_register_fn(
        c_registry,
        c_name.as_ptr(),
        Arc::into_raw(Arc::new(UserRepository::new(Arc::new(
            InMemoryUserRepository::new(),
        )))) as *const c_void as *mut c_void,
        dropper::<UserRepository>,
    ) < 0
    {
        panic!("Cannot register {}", c_name.to_str().unwrap());
    }

    let c_name = CString::from_str("user_internet_repository").unwrap();
    if registry_register_fn(
        c_registry,
        c_name.as_ptr(),
        Arc::into_raw(Arc::new(UserInternetRepository::new(Arc::new(
            InMemoryUserInternetRepository::new(),
        )))) as *const c_void as *mut c_void,
        dropper::<UserInternetRepository>,
    ) < 0
    {
        panic!("Cannot register {}", c_name.to_str().unwrap());
    }

    let c_name = CString::from_str("user_permission_repository").unwrap();
    if registry_register_fn(
        c_registry,
        c_name.as_ptr(),
        Arc::into_raw(Arc::new(UserPermissionRepository::new(Arc::new(
            InMemoryUserPermissionRepository::new(),
        )))) as *const c_void as *mut c_void,
        dropper::<UserPermissionRepository>,
    ) < 0
    {
        panic!("Cannot register {}", c_name.to_str().unwrap());
    }

    let c_name = CString::from_str("user_password_policy_repository").unwrap();
    if registry_register_fn(
        c_registry,
        c_name.as_ptr(),
        Arc::into_raw(Arc::new(UserPasswordPolicyRepository::new(Arc::new(
            InMemoryUserPasswordPolicyRepository::new(),
        )))) as *const c_void,
        dropper::<UserPasswordPolicyRepository>,
    ) < 0
    {
        panic!("Cannot register {}", c_name.to_str().unwrap());
    }

    let c_name = CString::from_str("user_password_repository").unwrap();
    if registry_register_fn(
        c_registry,
        c_name.as_ptr(),
        Arc::into_raw(Arc::new(UserPasswordRepository::new(Arc::new(
            InMemoryUserPasswordRepository::new(),
        )))) as *const c_void as *mut c_void,
        dropper::<UserPasswordRepository>,
    ) < 0
    {
        panic!("Cannot register {}", c_name.to_str().unwrap());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn plugin_initialize(
    c_registry: *const CRegistry,
    registry_get_fn: extern "C" fn(*const CRegistry, *const c_char, *mut *const c_void),
) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let c_name = CString::new("user_repository").unwrap(); // <- Fixed: correct name
        let mut user_repository_ptr: *const c_void = unsafe { zeroed() };
        registry_get_fn(c_registry, c_name.as_ptr(), &raw mut user_repository_ptr);

        if user_repository_ptr.is_null() {
            panic!("Cannot get user_repository");
        }

        // Clone the Arc to increment refcount, use it, then put it back
        let user_repository =
            unsafe { Arc::from_raw(user_repository_ptr as *const UserRepository) };
        let cloned = user_repository.clone();
        let _ = Arc::into_raw(user_repository); // Put it back!

        // Use the cloned Arc
        cloned.initialize().await.unwrap();
        // cloned is dropped here, but the original is still in the registry
    });
}
