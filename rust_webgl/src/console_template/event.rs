use crate::wasm_bindgen::JsCast;
use crate::Canvas;
use crate::Payload;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

// use super::{draw_layout::ImageLayout, draw_square::Square};
use super::draw_layout::ImageLayout;
pub fn on_wheel_event(
  canvas: Canvas,
  mut x: f32,
  mut y: f32,
  // scale: i32,z
  init: ImageLayout,
  square: Vec<Payload>,
  // square2: Square,
) -> Result<Canvas, JsValue> {
  {
    let canvas = canvas.clone();
    let canvas1 = canvas.canvas.clone();
    let wheel_callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
      x = x - (event.delta_x() / 2000.0) as f32;
      y = y + (event.delta_y() / 2000.0) as f32;
      for name in square.iter() {
        let _draw = canvas.draw(&init, &name, x, y, 0.0);
      }
    }) as Box<dyn FnMut(_)>);
    
    canvas1.add_event_listener_with_callback("wheel", wheel_callback.as_ref().unchecked_ref())?;
    wheel_callback.forget();
  }
  Ok(canvas)
}
