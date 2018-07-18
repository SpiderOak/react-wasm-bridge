#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate react_wasm_bridge;

use wasm_bindgen::prelude::*;
use react_wasm_bridge::{ State, Builder };
use std::f64;

const SVGNS: &str = "http://www.w3.org/2000/svg";

#[wasm_bindgen]
pub fn render(state: &State, builder: &Builder) -> JsValue {
    let slices = state.props_get_number("slices") as usize;

    builder.newContext("svg");
    builder.setNS(SVGNS);

    builder.setAttr("width", "256");
    builder.setAttr("height", "256");

    if slices > 0 && slices < 8 {
        builder.newContext("path");
        builder.setNS(SVGNS);
        let pr = (slices as f64) * (f64::consts::PI / 4.0);
        let x = 128.0 + 100.0 * pr.sin();
        let y = 128.0 - 100.0 * pr.cos();
        let af = if slices > 4 { 1 } else { 0 };
        builder.setAttr("d", &format!("M 128 128 L 128 28 A 100 100 0 {} 1 {} {} Z", af, x, y));
        builder.setAttr("fill", "red");
        builder.finishContext(); // path
    } else if slices >= 8 {
        builder.newContext("circle");
        builder.setNS(SVGNS);
        builder.setAttr("cx", "128");
        builder.setAttr("cy", "128");
        builder.setAttr("r", "100");
        builder.setAttr("fill", "red");
        builder.finishContext(); // path
    }

    builder.newContext("circle");
    builder.setNS(SVGNS);
    builder.setAttr("cx", "128");
    builder.setAttr("cy", "128");
    builder.setAttr("r", "100");
    builder.setAttr("fill", "none");
    builder.setAttr("stroke", "black");
    builder.setAttr("stroke-width", "2");
    builder.finishContext();

    builder.finishContext() // svg
}
