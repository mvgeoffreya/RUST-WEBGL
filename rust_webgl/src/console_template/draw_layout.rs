// use super::init::{compile_shader, link_program, Canvas};
use wasm_bindgen::prelude::*;
// use web_sys::console;
use web_sys::WebGlRenderingContext;
pub struct ImageLayout {
  pub vertices: Vec<f32>,
  // pub translation:web_sys::WebGlUniformLocation
}

impl ImageLayout {
  pub fn init_layout(
    ctx: &WebGlRenderingContext,
    translation: &web_sys::WebGlUniformLocation,
    // program: &web_sys::WebGlProgram,
    scale: i32,
    x: f32,
    y: f32,
    z: f32,
  ) -> Result<ImageLayout, JsValue> {
    let mut vertices = Vec::new();
    for points in 0..200 {
      vertices.push(-1.0 * scale as f32);
      vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
      // vertices.push(0.0);

      vertices.push(((100 - points) as f32 / 100.0) * scale as f32);
      vertices.push(1.0 * scale as f32);
      // vertices.push(0.0);

      vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
      vertices.push(-1.0 * scale as f32);
      // vertices.push(0.0);

      vertices.push(1.0 * scale as f32);
      vertices.push(((100 - points) as f32 / 100.0) * scale as f32);
      // vertices.push(0.0);

      vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
      vertices.push(1.0 * scale as f32);
      // vertices.push(0.0);

      vertices.push(1.0 * scale as f32);
      vertices.push(((points - 100) as f32 / 100.0) * scale as f32);
      // vertices.push(0.0);

      vertices.push(((-points + 100) as f32 / 100.0) * scale as f32);
      vertices.push(-1.0 * scale as f32);
      // vertices.push(0.0);

      vertices.push(-1.0 * scale as f32);
      vertices.push(((-points + 100) as f32 / 100.0) * scale as f32);
      // vertices.push(0.0);
    }
    // let vert_shader = compile_shader(
    //   &ctx,
    //   WebGlRenderingContext::VERTEX_SHADER,
    //   r#"
    //   attribute vec2 coordinates;
    //   uniform vec4 translation;
    //   void main() {
    //       gl_Position = vec4(coordinates, 0.0, 1.0) + translation;
    //       }
    //   "#,
    // )?;
    // let frag_shader = compile_shader(
    //   &ctx,
    //   WebGlRenderingContext::FRAGMENT_SHADER,
    //   r#"
    //   void main() {
    //       gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    //       }
    //     "#,
    // )?;
    //   let program = link_program(&ctx, &vert_shader, &frag_shader)?;
    //   ctx.use_program(Some(&program));
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
    // let location: u32 = ctx.get_attrib_location(&program, "coordinates") as u32;
    ctx.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    // ctx.enable_vertex_attrib_array(location);
    // let color = ctx.get_attrib_location(&program, "color") as u32;
    // ctx.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    // ctx.enable_vertex_attrib_array(color);
    // let translation = ctx
    //   .get_uniform_location(&program, "translation")
    //   .ok_or("failed to get uniform location")?;
    ctx.uniform4f(Some(&translation), x, y, z, 0.0);
    ctx.enable_vertex_attrib_array(0);
    ctx.draw_arrays(
      WebGlRenderingContext::LINES,
      0,
      (&vertices.len() / 2) as i32,
    );
    // Ok(ImageLayout { vertices, translation })
    
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
    // ctx.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    // ctx.enable_vertex_attrib_array(color);
    ctx.uniform4f(Some(translation), x, y, z, 0.0);
    ctx.enable_vertex_attrib_array(0);
    ctx.draw_arrays(
      WebGlRenderingContext::LINES,
      0,
      (vertices.len() / 2) as i32,
    );
    Ok(1)
}