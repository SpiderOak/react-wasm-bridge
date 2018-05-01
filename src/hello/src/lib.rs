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
                    Tag::Paragraph => builder.newContext("p".to_string()),
                    Tag::Rule => builder.newContext("hr".to_string()),
                    Tag::Header(level) => builder.newContext(format!("h{}",level).to_string()),

                    Tag::BlockQuote => builder.newContext("blockquote".to_string()),
                    Tag::CodeBlock(code) => {
                        builder.newContext("pre".to_string());
                        builder.newContext("code".to_string());
                        builder.addText(code.to_string());
                    },

                    // A list. If the list is ordered the field
                    // indicates the number of the first item.
                    Tag::List(index)=> {
                        match index {
                            Some(start) =>  {
                                builder.newContext("ol".to_string());
                                builder.setAttr("start".to_string(), start.to_string());
                            }
                            None =>  builder.newContext("ul".to_string()),
                        }
                    },
                    Tag::Item => builder.newContext("li".to_string()),
                    Tag::FootnoteDefinition(footer) => {
                        builder.newContext("footer".to_string());
                        builder.addText(footer.to_string());
                    },

                    // tables
                    // BUG: handle alignment on table
                    Tag::Table(_) => builder.newContext("table".to_string()),
                    Tag::TableHead => builder.newContext("th".to_string()),
                    Tag::TableRow => builder.newContext("tr".to_string()),
                    Tag::TableCell => builder.newContext("td".to_string()),

                    // span-level tags
                    Tag::Emphasis => builder.newContext("em".to_string()),
                    Tag::Strong => builder.newContext("strong".to_string()),
                    Tag::Code => builder.newContext("code".to_string()),

                    Tag::Link(url, title) => {
                        builder.newContext("a".to_string());
                        builder.setAttr("href".to_string(), url.to_string());
                        builder.setAttr("title".to_string(), title.to_string());
                    },

                    Tag::Image(url, title) => {
                        builder.newContext("img".to_string());
                        builder.setAttr("href".to_string(), url.to_string());
                        builder.setAttr("title".to_string(), title.to_string());
                    },
                }
            },
            Event::End(tag) => {
                match tag {
                    Tag::CodeBlock(_) => {
                        builder.finishContext();
                        builder.finishContext();
                    },
                    _ => {builder.finishContext();},
                }
            }
            Event::Html(text) => builder.addText(text.to_string()),
            Event::InlineHtml(text) => builder.addText(text.to_string()),
            Event::FootnoteReference(text) => builder.addText(text.to_string()),
            Event::SoftBreak => {
                builder.newContext("br".to_string());
                builder.finishContext();
                
            },
            Event::HardBreak => {
                builder.newContext("br".to_string());
                builder.finishContext();
                
            },
        }
    }
}
