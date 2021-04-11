extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::{draw_layout::draw_layout, draw_square::draw_square, init::Canvas};

// #[wasm_bindgen]
// pub fn inside(att_id: &str) -> Result<i32, JsValue> {
//   let canvas = Canvas::new(att_id)?;
//   let draw_square = draw_square(canvas);
//   Ok(draw_square?)
// }

// #[wasm_bindgen]
// pub fn base(att_id: &str, scale: i32) -> Result<i32, JsValue> {
//   let canvas = Canvas::new(att_id)?;
//   let draw_layout = draw_layout(canvas, scale);
//   Ok(draw_layout?)
// }
#[wasm_bindgen]
pub fn draw_it(att_id: &str, scale: i32) -> Result<i32, JsValue> {
  let canvas = Canvas::new(att_id)?;
  let draw_square = draw_square(&canvas, scale);
  let draw_layout = draw_layout(&canvas, scale);
  Ok(draw_layout?)
}
