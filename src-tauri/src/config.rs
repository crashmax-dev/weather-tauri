// https://github.com/ElmTran/praises/blob/333fbf18625d9b4da17dc5c57005e18cc208cb7e/src-tauri/src/app.rs

use crate::app::HANDLE;
use current_locale;
use dirs::config_dir;
use log::{ info, warn };
use serde_json::{ json, Value };
use std::sync::Mutex;
use tauri::{ Manager, Wry };
use tauri_plugin_store::{ Store, StoreBuilder };

pub struct StoreWrapper(pub Mutex<Store<Wry>>);

pub fn get_locale() -> String {
  let current_locale = current_locale::current_locale().unwrap_or(String::from("ru"));
  current_locale
}

pub fn init_config(app: &mut tauri::App) {
    let config_path = config_dir().unwrap();
    let config_path = config_path.join(app.config().tauri.bundle.identifier.clone());
    let config_path = config_path.join(".config.dat");
    info!("Load config from: {:?}", config_path);
    let mut store = StoreBuilder::new(app.handle(), config_path).build();

    match store.load() {
        Ok(_) => info!("Config loaded"),
        Err(e) => {
            warn!("Config load error: {:?}", e);
            info!("Config not found, creating new config");
        }
    }
    app.manage(StoreWrapper(Mutex::new(store)));
}

pub fn get(key: &str) -> Option<Value> {
    let state = HANDLE.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    match store.get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn set<T: serde::ser::Serialize>(key: &str, value: T) {
    let state = HANDLE.get().unwrap().state::<StoreWrapper>();
    let mut store = state.0.lock().unwrap();
    store.insert(key.to_string(), json!(value)).unwrap();
    store.save().unwrap();
}

pub fn is_first_run() -> bool {
    let state = HANDLE.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    store.is_empty()
}
