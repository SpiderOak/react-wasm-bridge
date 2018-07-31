extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "react-wasm-bridge", version="0.1.0")]
extern {
    pub type Builder;
    
    #[wasm_bindgen(method)]
    pub fn newContext(this: &Builder, name: &str);
    
    #[wasm_bindgen(method)]
    pub fn addText(this: &Builder, text: &str);

    #[wasm_bindgen(method)]
    pub fn setAttr(this: &Builder, key: &str, value: &str);

    #[wasm_bindgen(method)]
    pub fn setNS(this: &Builder, value: &str);

    #[wasm_bindgen(method)]
    pub fn finishContext(this: &Builder) -> JsValue;
}
