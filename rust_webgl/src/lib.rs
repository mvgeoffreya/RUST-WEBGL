extern crate wasm_bindgen;
extern crate web_sys;
// use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::{
  draw_layout::ImageLayout, draw_square::Square, event::on_wheel_event, init::Canvas,
};


#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Payload {
  pub vertices: Vec<f32>,
  pub colors: Vec<f32>,
  pub indices: Vec<u16>
}


#[wasm_bindgen]
pub fn draw_it(att_id: &str, scale: i32, payload:&JsValue) -> Result<i32, JsValue> {
  let payload:Vec<Payload>  = payload.into_serde().unwrap();
  let x: f32 = 0.0;
  let y: f32 = 0.0;
  let canvas = Canvas::new(att_id)?;
  let init_layout = ImageLayout::init_layout(&canvas.ctx, &canvas.translation, scale, x, y, 0.0)?;
  let _init_square = Square::init_square(&payload,&canvas.ctx, &canvas.translation, canvas.color, scale, x, y, 0.0)?;
  let _connect = on_wheel_event(canvas, x, y, init_layout, payload );
  Ok(1)
}
