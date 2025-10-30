mod user;

use std::{path::PathBuf, sync::Arc};

use fototra::{
    adapters::{Adapter, repository::in_memory::InMemoryRepository},
    runtime::{Runtime, get_runtime},
};
use libloading::{Library, Symbol};
use user::test_users;

#[tokio::test]
async fn tests() {
    let mut lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
    let mut get_runtime: Option<Symbol<unsafe extern "C" fn() -> Runtime>> = None;
    unsafe {
        let get_adapter: Symbol<unsafe extern "C" fn() -> Adapter> = lib
            .get(b"get_adapter")
            .expect("Failed to load 'get_adapter' function");
        get_runtime = Some(
            lib.get(b"get_runtime")
                .expect("Failed to load 'get_adapter' function"),
        );

        get_adapter().insert(Arc::new(InMemoryRepository)).await;
        get_runtime.as_ref().unwrap()().init().await.unwrap();
    }

    test_users().await;

    unsafe { get_runtime.as_ref().unwrap()().end().await }
}
