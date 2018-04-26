extern crate json;

use std::collections::HashMap;

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

	fn to_json_value(&self) -> json::JsonValue {
		let mut json_attributes = json::JsonValue::new_object();
		for (k, v) in &self.attributes {
			json_attributes[k] = json::JsonValue::String(v.to_string());
		}

		json::object!{
			"type" => "element",
			"name" => self.name.clone(),
			"attributes" => json_attributes,
			"children" => self.children.iter().map(|n| { 
				match n {
					Node::Element(e) => e.to_json_value(),
					Node::Text(t) => json::object!{
						"type" => "text",
						"text" => t.clone(),
					}
				}
			}).collect::<Vec<json::JsonValue>>(),
		}
	}

	pub fn to_json(&mut self) -> String {
		json::stringify(self.to_json_value())
	}
}
