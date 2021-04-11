extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::draw::draw_layout;
use console_template::draw::Canvas;

#[wasm_bindgen]

pub fn start(att_id: &str, scale: i32) -> Result<i32, JsValue> {
  let canvas = Canvas::new(att_id)?;
  let location = draw_layout(canvas, scale)?;
  Ok(location)
}
