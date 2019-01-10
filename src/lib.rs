mod builder;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

pub use crate::builder::*;

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

	pub fn props_get_string(&self, k: &str) -> String {
		match self.props.get(k) {
			Some(p) => match p {
				PropValue::S(v) => v.clone(),
				_ => "".to_string(),
			},
			None => "".to_string(),
		}
	}

	pub fn props_get_number(&self, k: &str) -> f64 {
		match self.props.get(k) {
			Some(p) => match p {
				PropValue::N(v) => *v,
				_ => 0.0,
			},
			None => 0.0,
		}
	}
}
