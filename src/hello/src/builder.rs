#![feature(proc_macro, wasm_custom_section, wasm_import_module, proc_macro_path_invoc)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub type Builder;
    
    #[wasm_bindgen(method)]
    pub fn factory(this: &Builder, name: String) -> Builder;
    
    #[wasm_bindgen(method)]
    pub fn addChild(this: &Builder, child: JsValue);

    #[wasm_bindgen(method)]
    pub fn addText(this: &Builder, text: String);

    #[wasm_bindgen(method)]
    pub fn setAttr(this: &Builder, key: String, value: String);

    #[wasm_bindgen(method)]
    pub fn finish(this: &Builder) -> JsValue;
}
