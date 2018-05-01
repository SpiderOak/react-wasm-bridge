#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate react_wasm_bridge;
extern crate pulldown_cmark;

use pulldown_cmark::Parser;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;

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

        render_markdown( &message, builder );

        builder.finishContext(); //li
    }

    builder.finishContext() //ul
}

fn render_markdown ( md: &str, builder: &Builder ) {
    let parser = Parser::new(md);
    
    for event in parser {
        match event {
            Event::Text(text) => builder.addText(&text.clone()),
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => builder.newContext("p"),
                    Tag::Rule => builder.newContext("hr"),
                    Tag::Header(level) => builder.newContext(&format!("h{}",level)),

                    Tag::BlockQuote => builder.newContext("blockquote"),
                    Tag::CodeBlock(code) => {
                        builder.newContext("pre");
                        builder.newContext("code");
                        builder.addText(&code.clone());
                    },

                    // A list. If the list is ordered the field
                    // indicates the number of the first item.
                    Tag::List(index)=> {
                        match index {
                            Some(start) =>  {
                                builder.newContext("ol");
                                builder.setAttr("start", &start.to_string());
                            }
                            None =>  builder.newContext("ul"),
                        }
                    },
                    Tag::Item => builder.newContext("li"),
                    Tag::FootnoteDefinition(footer) => {
                        builder.newContext("footer");
                        builder.addText(&footer.clone());
                    },

                    // BUG: Alingment not implmented
                    Tag::Table(_) => builder.newContext("table"),
                    Tag::TableHead => builder.newContext("th"),
                    Tag::TableRow => builder.newContext("tr"),
                    Tag::TableCell => builder.newContext("td"),

                    // span-level tags
                    Tag::Emphasis => builder.newContext("em"),
                    Tag::Strong => builder.newContext("strong"),
                    Tag::Code => builder.newContext("code"),

                    Tag::Link(url, title) => {
                        builder.newContext("a");
                        builder.setAttr("href", &url.clone());
                        builder.setAttr("title", &title.clone());
                    },

                    Tag::Image(url, title) => {
                        builder.newContext("img");
                        builder.setAttr("href", &url.clone());
                        builder.setAttr("title", &title.clone());
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
            Event::Html(text) => builder.addText(&text.clone()),
            Event::InlineHtml(text) => builder.addText(&text.clone()),
            Event::FootnoteReference(text) => builder.addText(&text.clone()),
            Event::SoftBreak => {
                builder.newContext("br");
                builder.finishContext();
                
            },
            Event::HardBreak => {
                builder.newContext("br");
                builder.finishContext();
                
            },
        }
    }
}
