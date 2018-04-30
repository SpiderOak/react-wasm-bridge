#![feature(proc_macro, wasm_custom_section, wasm_import_module, proc_macro_path_invoc)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use builder::*;

    
pub enum Node {
	Text(String),
	Element(Element),
}


pub struct Element {
	name: String,
	attributes: HashMap<String, String>,
	children: Vec<Node>,
}


impl Element {
	pub fn new(name: &str) -> Element {
		Element {
			name: name.to_string(),
			attributes: HashMap::new(),
			children: Vec::new(),
		}
	}

	pub fn attr_set(&mut self, k: &str, v: &str) {
		self.attributes.insert(k.to_string(), v.to_string());
	}

	pub fn child_add(&mut self, child: Node) {
		self.children.push(child);
	}

    
    pub fn render(&self, factory: &Builder) -> JsValue {

        let builder = factory.factory(self.name.clone());

        for (k, v) in &self.attributes {
	    builder.setAttr(k.to_string(),  v.to_string());
	}

        for node in &self.children {
            match node {
		Node::Text(t) => builder.addText( t.clone() ),
		Node::Element(e) => builder.addChild( e.render(factory) ),
		}
	    }
    
        builder.finish()
    }
}
