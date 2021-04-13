extern crate wasm_bindgen;
extern crate web_sys;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
mod console_template;
use console_template::{
  draw_layout::init_layout, draw_square::init_square, init::draw_lay, init::draw_sq, init::Canvas,
};
// use web_sys::console;

#[wasm_bindgen]
pub fn draw_it(att_id: &str, scale: i32) -> Result<f32, JsValue> {
  let x: f32 = 0.0;
  let y: f32 = 0.0;
  let canvas = Canvas::new(att_id)?;
  // let ctx = canvas.ctx;
  // let canvas = canvas.canvas;
  // let ctx = Rc::new(ctx);
  // let canvas = Rc::new(canvas);
  // let canvas = Rc::new(canvas);
 
  {
    // let canvas = canvas.clone();
    let init_square = init_square(&canvas, scale, x, y, 0.0)?;
    draw_sq(canvas, x, y, 0.0, scale, init_square);
  }
  {  
    // let canvas = canvas.clone();
    // let init_layout = init_layout(&canvas, scale, x, y, 0.0)?;
    // let init_layout =draw_lay(canvas, x, y, 0.0, scale, init_layout);
  }
  // on_wheel_event(canvas,x,y,scale)?;
  Ok(x)
}
