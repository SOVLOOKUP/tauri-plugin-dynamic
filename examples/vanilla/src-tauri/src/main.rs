#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  plugin::{Plugin, TauriPlugin},
  Runtime,
};

fn load_plugin<R: Runtime>(path: &str) -> Result<TauriPlugin<R>, Box<dyn std::error::Error>> {
  unsafe {
    let lib = libloading::Library::new(path)?;
    let func: libloading::Symbol<fn() -> TauriPlugin<R>> = lib.get(b"init")?;
    let plugin = func();
    println!("plugin name: {}", plugin.name());
    Ok(plugin)
  }
}

fn main() {
  let plugin = load_plugin("../../../target/release/libtauri_plugin_test.so").unwrap();
  println!("something went wrong here!!");
  // segmentation fault (core dumped)
  println!("{}", plugin.name());
  tauri::Builder::default()
    .plugin(plugin)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
