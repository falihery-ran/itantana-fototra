mod user;

use std::{path::PathBuf, sync::Arc};

use fototra::{adapters::repository::in_memory::InMemoryRepository, runtime::Runtime};
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
        get_runtime = Some(
            lib.get(b"get_runtime")
                .expect("Failed to load 'get_adapter' function"),
        );

        get_runtime.as_ref().unwrap()().init().await.unwrap();
        get_runtime.as_ref().unwrap()()
            .add_adapter(Arc::new(InMemoryRepository))
            .await;
    }

    test_users().await;
}
