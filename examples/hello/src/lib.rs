#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate react_wasm_bridge;

use wasm_bindgen::prelude::*;
use react_wasm_bridge::{ State, Builder };

#[wasm_bindgen]
pub fn render(state: &State, builder: &Builder) -> JsValue {

    let x = state.props_get_number("x") as usize;
    let message = state.props_get_string("message");

    builder.newContext("ul");

    builder.setAttr("className", "output");

    for k in 0..x {
        builder.newContext("li");

	builder.setAttr("key", &k.to_string());

        builder.addText(&message.clone());

        builder.finishContext(); //li
    }

    builder.finishContext() //ul
}
