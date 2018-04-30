#![feature(proc_macro, wasm_custom_section, wasm_import_module, proc_macro_path_invoc)]

mod element;
mod builder;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use element::*;
pub use builder::*;

pub enum PropValue {
	S(String),
	N(f64),
}

#[wasm_bindgen]
pub struct State {
	props: HashMap<String, PropValue>,
}

#[wasm_bindgen]
impl State {
	pub fn new() -> State {
		State { props: HashMap::new() }
	}

	pub fn props_clear(&mut self) {
		self.props.clear();
	}

	pub fn props_set_string(&mut self, k: &str, v: &str) {
		self.props.insert(k.to_string(), PropValue::S(v.to_string()));
	}

	pub fn props_set_number(&mut self, k: &str, v: f64) {
		self.props.insert(k.to_string(), PropValue::N(v));
	}
}

#[wasm_bindgen]
pub fn render(state: &State, factory: &Builder) -> JsValue {
    let x = *match state.props.get("x").unwrap() {
	PropValue::N(x) => x,
	_ => &0.0,
    } as usize;

    let message = match state.props.get("message").unwrap() {
	PropValue::S(x) => x,
	_ => "blonk",
    };

    let mut elem = Element::new("ul");

    elem.attr_set("className", "output");

    for k in 0..x {
	let mut li = Element::new("li");
	li.attr_set("key", &k.to_string());
	li.child_add(Node::Text(message.to_string()));

	elem.child_add(Node::Element(li));
    }

    elem.render(factory)
}
