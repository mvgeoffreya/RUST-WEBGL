use std::rc::Rc;
use super::init::{compile_shader, link_program, Canvas};
use crate::wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::WebGlRenderingContext;

pub struct GridLayout {
  pub translation: web_sys::WebGlUniformLocation, 
  pub vertices:Vec<f32>
}

pub fn init_layout(canvas: &Canvas, scale: i32, mut x: f32, mut y: f32, z: f32) -> Result<GridLayout, JsValue> {
  let mut vertices = Vec::new();
  let ctx = &canvas.ctx;
  for points in 0..200 {
    vertices.push(-1.0 * scale as f32);
    vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
    vertices.push(((100 - points) as f32 / 100.0) * scale as f32);
    vertices.push(1.0 * scale as f32);

    vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
    vertices.push(-1.0 * scale as f32);
    vertices.push(1.0 * scale as f32);
    vertices.push(((100 - points) as f32 / 100.0) * scale as f32);

    vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
    vertices.push(1.0 * scale as f32);
    vertices.push(1.0 * scale as f32);
    vertices.push(((points - 100) as f32 / 100.0) * scale as f32);

    vertices.push(((-points + 100) as f32 / 100.0) * scale as f32);
    vertices.push(-1.0 * scale as f32);
    vertices.push(-1.0 * scale as f32);
    vertices.push(((-points + 100) as f32 / 100.0) * scale as f32);
  }
  let vert_shader = compile_shader(
    &ctx,
    WebGlRenderingContext::VERTEX_SHADER,
    r#"
    attribute vec2 coordinates;
    uniform vec4 translation;
    void main() {
        gl_Position = vec4(coordinates, 0.0, 1.0) + translation;
        }
    "#,
  )?;
  let frag_shader = compile_shader(
    &ctx,
    WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
    void main() {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
      "#,
  )?;
  let program = link_program(&ctx, &vert_shader, &frag_shader)?;
  ctx.use_program(Some(&program));
  let translation = ctx
    .get_uniform_location(&program, "translation")
    .ok_or("failed to get uniform location")?;
  ctx.uniform4f(Some(&translation), x, y, z, 0.0);
  let buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
  unsafe {
    let vert_array = js_sys::Float32Array::view(&vertices);

    ctx.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &vert_array,
      WebGlRenderingContext::STATIC_DRAW,
    );
  }
  let location = ctx.get_attrib_location(&program, "coordinates");
  ctx.vertex_attrib_pointer_with_i32(
    location as u32,
    2,
    WebGlRenderingContext::FLOAT,
    false,
    0,
    0,
  );
  ctx.enable_vertex_attrib_array(0);
  ctx.draw_arrays(WebGlRenderingContext::LINES, 0, (vertices.len() / 2) as i32);
  
  // {
  //   let ctx = Rc::new(ctx);
  //   let ctx = ctx.clone();
  //   let wheel_callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
  //     x = x - (event.delta_x() / 1000.0) as f32;
  //     y = y + (event.delta_y() / 1000.0) as f32;
  //     ctx.uniform4f(Some(&translation), x, y, z, 0.0);
  //     ctx.draw_arrays(WebGlRenderingContext::LINES, 0, (vertices.len() / 2) as i32);
  //     console::log_1(&x.into());
  //   }) as Box<dyn FnMut(_)>);
  //   canvas
  //     .canvas
  //     .add_event_listener_with_callback("wheel", wheel_callback.as_ref().unchecked_ref())?;
  //     wheel_callback.forget();
  // }
  let location = ctx.get_attrib_location(&program, "coordinates");
  Ok(GridLayout {
    translation,
    vertices
  })
}


pub fn draw_layout(ctx:&WebGlRenderingContext, x:f32, y:f32, z:f32, translation: web_sys::WebGlUniformLocation, vertices:Vec<f32>) {
  ctx.uniform4f(Some(&translation), x, y, z, 0.0);
  ctx.draw_arrays(WebGlRenderingContext::LINES, 0, (vertices.len() / 2) as i32);
}