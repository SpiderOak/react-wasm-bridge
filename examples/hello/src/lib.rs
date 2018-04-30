#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate react_wasm_bridge;

use wasm_bindgen::prelude::*;
use react_wasm_bridge::{ State, Builder };

#[wasm_bindgen]
pub fn render(state: &State, factory: &Builder) -> JsValue {
    let x = state.props_get_number("x") as usize;
    let message = state.props_get_string("message");

    let elem = factory.factory("ul".to_string());

    elem.setAttr("className".to_string(), "output".to_string());

    for k in 0..x {
	//let mut li = Element::new("li");
        let li = factory.factory("li".to_string());

	li.setAttr("key".to_string(), k.to_string());

	li.addText(message.to_string());

	elem.addChild(li.finish());
    }

    elem.finish()
}
