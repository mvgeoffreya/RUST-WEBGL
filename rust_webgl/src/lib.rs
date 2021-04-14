extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::{
  draw_layout::ImageLayout, draw_square::Square, event::on_wheel_event, init::Canvas,
};
// use web_sys::console;

#[wasm_bindgen]
pub fn draw_it(att_id: &str, scale: i32) -> Result<i32, JsValue> {
  let x: f32 = 0.0;
  let y: f32 = 0.0;
  let canvas = Canvas::new(att_id)?;
  let init_layout = ImageLayout::init_layout(&canvas.ctx, &canvas.translation, scale, x, y, 0.0)?;
  let init_square = Square::init_square(&canvas.ctx, &canvas.translation, canvas.color, scale, x, y, 0.0)?;
  let _connect = on_wheel_event(canvas, x, y, init_layout, init_square);
  // let init_square = Square::init_square(&canvas.ctx, &canvas.translation, scale, x, y, 0.0)?;
  // canvas.draw_layout(&init, x, y, 0.0);
  // let _connect = on_wheel_event(canvas, x, y, scale, init_layout);
  Ok(1)
}
