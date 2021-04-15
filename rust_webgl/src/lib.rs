extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::{
  draw_layout::ImageLayout, draw_square::Square, event::on_wheel_event, init::Canvas,
};

pub struct Payload {
  pub vertices: Vec<f32>,
  pub colors: Vec<f32>,
  pub indices: Vec<u16>
}

#[wasm_bindgen]
pub fn draw_it(att_id: &str, scale: i32) -> Result<i32, JsValue> {
  let x: f32 = 0.0;
  let y: f32 = 0.0;
  let canvas = Canvas::new(att_id)?;
  let vertices1: Vec<f32> = [
    -(0.05 / 10.0) * scale as f32,
    (0.05 / 10.0) * scale as f32,
    0.00,
    -(0.1 / 10.0) * scale as f32,
    -(0.00 / 10.0) * scale as f32,
    0.00,
    -(0.05 / 10.0) * scale as f32,
    -(0.05 / 10.0) * scale as f32,
    0.00,
    (0.00 / 10.0) * scale as f32,
    (0.00 / 10.0) * scale as f32,
    0.00,
  ]
  .to_vec();
  let colors1: Vec<f32> = [0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0, 0.1, 0.0, 0.1].to_vec();
  let indices1: Vec<u16> = [3, 2, 1, 3, 1, 0].to_vec();
  let vertices2: Vec<f32> = [
    -(1.5 / 10.0) * scale as f32,
    (1.5 / 10.0) * scale as f32,
    0.00,
    -(1.45 / 10.0) * scale as f32,
    (1.45 / 10.0) * scale as f32,
   0.00,
    (0.7 / 10.0) * scale as f32,
    (0.7 / 10.0) * scale as f32,
   0.00,
    (1.45 / 10.0) * scale as f32,
    (1.45 / 10.0) * scale as f32,
   0.00
  ]
  .to_vec();
  let colors2: Vec<f32> = [0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0, 0.1, 0.0, 0.1].to_vec();
  let indices2: Vec<u16> = [3, 2, 1, 3, 1, 0].to_vec();
  let mut payload = Vec::new();
  payload.push(Payload {
    vertices:vertices1,
    colors: colors1,
    indices: indices1,
   });
   payload.push(Payload {
    vertices:vertices2,
    colors: colors2,
    indices: indices2,
   });
   
  
  let init_layout = ImageLayout::init_layout(&canvas.ctx, &canvas.translation, scale, x, y, 0.0)?;
  let _init_square = Square::init_square(&payload,&canvas.ctx, &canvas.translation, canvas.color, scale, x, y, 0.0)?;
  let _connect = on_wheel_event(canvas, x, y, init_layout, payload );
  Ok(1)
}
