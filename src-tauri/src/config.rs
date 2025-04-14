// https://github.com/ElmTran/praises/blob/333fbf18625d9b4da17dc5c57005e18cc208cb7e/src-tauri/src/app.rs

use crate::app::HANDLE;
use current_locale;
use dirs::config_dir;
use log::info;
use serde_json::{json, Value};
use std::sync::Arc;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};

pub struct StoreWrapper(pub Arc<Store<Wry>>);

pub fn get_lang() -> String {
    let current_locale = current_locale::current_locale().unwrap_or(String::from("ru"));
    current_locale
}

pub fn init_config(app: &mut tauri::App) {
    let config_path = config_dir().unwrap();
    let config_path = config_path.join(app.config().identifier.clone());
    let config_path = config_path.join(".config.dat");
    info!("Load config from: {:?}", config_path);
    let store = StoreBuilder::new(app.handle(), config_path)
        .build()
        .unwrap();

    app.manage(StoreWrapper(store));
}

pub fn get(key: &str) -> Option<Value> {
    let state = HANDLE.get().unwrap().state::<StoreWrapper>();
    let store = &state.0;
    match store.get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn set<T: serde::ser::Serialize>(key: &str, value: T) {
    let state = HANDLE.get().unwrap().state::<StoreWrapper>();
    let store = &state.0;
    store.set(key.to_string(), json!(value));
    store.save().unwrap();
}

pub fn is_first_run() -> bool {
    let state = HANDLE.get().unwrap().state::<StoreWrapper>();
    let store = &state.0;
    store.is_empty()
}
