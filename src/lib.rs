use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[no_mangle]
pub fn init() -> TauriPlugin<impl Runtime> {
    Builder::<tauri::Wry>::new("test").build()
}
