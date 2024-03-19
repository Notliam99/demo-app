// use my_demo_app_ui::get_os;
// // use serde::{Deserialize, Serialize};
// // use serde_wasm_bindgen::to_value;
// use tauri_invoke::invoke;
// use wasm_bindgen::prelude::*;
// // use wasm_bindgen_futures::spawn_local;
// // use yew::{platform::spawn_local, prelude::*};
//
// invoke!(pub async fn get_os(cmd: "get_os", args:) -> String);
//
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn get_os() -> JsValue;
// }
