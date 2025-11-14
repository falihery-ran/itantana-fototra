use std::{env, path::PathBuf};

use fototra::{c_registry::CRegistry, registry::Registry};
use libloading::{Library, Symbol};

use crate::user::test_users;

mod user;

#[tokio::main]
async fn main() {
    let mut lib_path =
        PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
            match env::var("CARGO_MANIFEST_DIR") {
                Ok(s) => PathBuf::from(s).parent().unwrap().display().to_string(),
                Err(_e) => env::current_dir().unwrap().to_str().unwrap().to_string(),
            }
        }));
    lib_path.push("target");
    lib_path.push(if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    });

    #[cfg(target_os = "windows")]
    lib_path.push("libfototra.dll");

    #[cfg(target_os = "macos")]
    lib_path.push("libfototra.dylib");

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    lib_path.push("libfototra.so");

    let lib = unsafe { Library::new(lib_path).expect("Failed to load library") };
    let mut get_c_registry: Option<Symbol<unsafe extern "C" fn() -> *mut CRegistry>> = None;
    unsafe {
        get_c_registry = Some(
            lib.get(b"c_registry_new")
                .expect("Failed to load 'c_registry_new' function"),
        );
    }

    let c_registry = unsafe { get_c_registry.unwrap()() };
    let registry = Registry::try_from_c_registry(c_registry).unwrap();

    registry.persist();
    let registry = Registry::get_instance();
    registry.init().await.unwrap();

    test_users().await;
}
