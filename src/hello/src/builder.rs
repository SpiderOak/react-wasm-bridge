#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "./../ReactWasmBridge")]
extern {
    pub type Builder;
    
    #[wasm_bindgen(method)]
    pub fn newContext(this: &Builder, name: &str);
    
    #[wasm_bindgen(method)]
    pub fn addText(this: &Builder, text: &str);

    #[wasm_bindgen(method)]
    pub fn setAttr(this: &Builder, key: &str, value: &str);

    #[wasm_bindgen(method)]
    pub fn finishContext(this: &Builder) -> JsValue;
}
