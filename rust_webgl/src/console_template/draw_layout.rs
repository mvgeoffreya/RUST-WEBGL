use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;
pub struct ImageLayout {
  pub vertices: Vec<f32>,
}

impl ImageLayout {
  pub fn init_layout(
    ctx: &WebGlRenderingContext,
    translation: &web_sys::WebGlUniformLocation,
    scale: i32,
    x: f32,
    y: f32,
    z: f32,
  ) -> Result<ImageLayout, JsValue> {
    let mut vertices = Vec::new();
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
    ctx.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    ctx.uniform4f(Some(&translation), x, y, z, 0.0);
    ctx.enable_vertex_attrib_array(0);
    ctx.draw_arrays(
      WebGlRenderingContext::LINES,
      0,
      (&vertices.len() / 2) as i32,
    );
    
    Ok(ImageLayout { vertices })
  }
}

pub fn draw_layout(ctx: &web_sys::WebGlRenderingContext,translation: &web_sys::WebGlUniformLocation, vertices: &Vec<f32>, x: f32, y: f32, z: f32 ) -> Result<i32,JsValue> {
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
    ctx.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    ctx.uniform4f(Some(translation), x, y, z, 0.0);
    ctx.enable_vertex_attrib_array(0);
    ctx.draw_arrays(
      WebGlRenderingContext::LINES,
      0,
      (vertices.len() / 2) as i32,
    );
    Ok(1)
}