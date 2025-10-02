use js_sys::wasm_bindgen::JsValue;

#[cfg(not(feature = "server"))]
pub fn eval(js: &str) -> JsValue {
    let result = js_sys::eval(js).unwrap();
    result
}

#[cfg(feature = "server")]
pub fn eval(_js: &str) -> JsValue {
    JsValue::NULL
}
