use super::init::{compile_shader, link_program, Canvas};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

pub fn draw_layout(canvas: &Canvas, scale: i32) -> Result<i32, JsValue> {
  let mut vertices = Vec::new();
  let ctx: &WebGlRenderingContext = &canvas.ctx;
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
    void main() {
        gl_Position = vec4(coordinates, 0.0, 1.0);
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
  let location = ctx.get_attrib_location(&program, "coordinates");
  Ok(location)
}
