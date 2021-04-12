use crate::draw_layout;
use crate::draw_square;
use crate::wasm_bindgen::JsCast;
use crate::Canvas;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::console;
pub struct Movement {
  pub x: f32,
  pub y: f32,
}

pub fn on_wheel_event(
  canvas: &Canvas,
  mut x: f32,
  mut y: f32,
  scale: i32,
) -> Result<Movement, JsValue> {
  let wheel_callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
    x = x - (event.delta_x() / 1000.0) as f32;
    y = y - (event.delta_y() / 1000.0) as f32;
    console::log_1(&x.into());
  }) as Box<dyn FnMut(_)>);
  canvas
    .canvas
    .add_event_listener_with_callback("wheel", wheel_callback.as_ref().unchecked_ref())?;
  let _draw_square = draw_square(&canvas, scale, x, y, 0.0);
  let _draw_layout = draw_layout(&canvas, scale, x, y, 0.0);
  wheel_callback.forget();

  Ok(Movement { x, y })
}
