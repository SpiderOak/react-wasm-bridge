#![feature(proc_macro, wasm_custom_section, wasm_import_module, proc_macro_path_invoc)]

mod builder;

extern crate wasm_bindgen;
extern crate pulldown_cmark;

use wasm_bindgen::prelude::*;
use pulldown_cmark::Parser;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;
use std::collections::HashMap;

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
pub fn render(state: &State, builder: &Builder) -> JsValue {
    let x = *match state.props.get("x").unwrap() {
	PropValue::N(x) => x,
	_ => &0.0,
    } as usize;

    let message = match state.props.get("message").unwrap() {
	PropValue::S(x) => x,
	_ => "blonk",
    };

    builder.newContext("ul".to_string());
    
    builder.setAttr("className".to_string(), "output".to_string());

    for k in 0..x {
        builder.newContext("li".to_string());

	builder.setAttr("key".to_string(), k.to_string());

        render_markdown( message, builder );

        builder.finishContext(); //li
    }

    builder.finishContext() //ul
}

fn render_markdown ( md: &str, builder: &Builder ) {
    let parser = Parser::new(md);
    
    for event in parser {
        match event {
            Event::Text(text) => builder.addText(text.to_string()),
            Event::Start(tag) => {
                match tag {
                    Tag::Emphasis => builder.newContext("b".to_string()),
                    _ => ()
                }
            },
            Event::End(tag) => {
                match tag {
                    Tag::Emphasis => {builder.finishContext();},
                    _ => ()
                }
            }
            _ => builder.addText("-MD-".to_string()),
        }
    }
}
