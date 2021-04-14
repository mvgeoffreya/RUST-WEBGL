use crate::wasm_bindgen::JsCast;
use crate::Canvas;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use std::rc::Rc;
// use web_sys::console;

use super::draw_layout::ImageLayout;
pub struct Movement {
  pub x: f32,
  pub y: f32,
}

pub fn on_wheel_event(
  canvas: Canvas,
  mut x: f32,
  mut y: f32,
  // scale: i32,
  init:ImageLayout
) -> Result<Movement, JsValue> {
  let canvas = Rc::new(canvas);
  let canvas1 = canvas.clone();
  let wheel_callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
    x = x - (event.delta_x() / 2000.0) as f32;
    y = y + (event.delta_y() / 2000.0) as f32;
    let _draw = canvas.draw_layout(&init, x, y, 0.0);
  }) as Box<dyn FnMut(_)>);
  canvas1
    .canvas
    .add_event_listener_with_callback("wheel", 
    wheel_callback.as_ref().unchecked_ref()
  )?;
  wheel_callback.forget();

  Ok(Movement { x, y })
}
