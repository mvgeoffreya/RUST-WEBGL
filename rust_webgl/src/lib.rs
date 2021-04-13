extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::{draw_layout::draw_layout, draw_square::draw_square, init::Canvas, event::on_wheel_event};
// use web_sys::console;

#[wasm_bindgen]
pub fn draw_it(att_id: &str, scale: i32 ) -> Result<f32, JsValue> {
  let x:f32=0.0; 
  let y:f32=0.0;
  let canvas = Canvas::new(att_id)?;
  on_wheel_event(canvas,x,y,scale)?;
  Ok(x)
}


