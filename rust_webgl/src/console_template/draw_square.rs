use super::init::{Canvas, compile_shader, link_program};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

pub fn draw_square(canvas: &Canvas, scale: i32) -> Result<i32, JsValue> {
  let colors = [0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0, 0.1, 0.0, 0.1];
  let ctx: &WebGlRenderingContext = &canvas.ctx;
  // colors
  let color_buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
  unsafe {
    let color_array = js_sys::Float32Array::view(&colors);
    ctx.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &color_array,
      WebGlRenderingContext::STATIC_DRAW,
    );
  }
  // colors
  // vertex
  let vertices = [
    -0.05*scale as f32, 0.05*scale as f32, 0.00, 
    -0.1*scale as f32, -0.00*scale as f32, 0.00,
     -0.05*scale as f32, -0.05*scale as f32, 0.00, 
     0.00*scale as f32, 0.00*scale as f32, 0.00,
  ];

  let vertex_buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
  unsafe {
    let vert_array = js_sys::Float32Array::view(&vertices);
    ctx.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &vert_array,
      WebGlRenderingContext::STATIC_DRAW,
    );
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER,None);
  }
  // vertex
  // index
  let indices = [3, 2, 1, 3, 1, 0];
  let index_buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
  ctx.bind_buffer(
    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
    Some(&index_buffer),
  );
  unsafe {
    let indice_array = js_sys::Uint16Array::view(&indices);
    ctx.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
      &indice_array,
      WebGlRenderingContext::STATIC_DRAW,
    );
    ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,None);
  }
  //compile
  let vert_shader = compile_shader(
    &ctx,
    WebGlRenderingContext::VERTEX_SHADER,
    r#"
      attribute vec3 coordinates;
      attribute vec3 color;
      varying vec3 vColor;
      void main() {
          gl_Position = vec4(coordinates, 1.0);
          vColor = color;
      }
  "#,
  )?;
  let frag_shader = compile_shader(
    &ctx,
    WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
      precision mediump float;
      varying vec3 vColor;
      void main() {
        gl_FragColor = vec4(vColor, 1.);
      }
  "#,
  )?;
  let program = link_program(&ctx, &vert_shader, &frag_shader)?;
  ctx.use_program(Some(&program));
  

  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
  ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

  let location: u32 = ctx.get_attrib_location(&program, "coordinates") as u32;
  ctx.vertex_attrib_pointer_with_i32(location, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
  ctx.enable_vertex_attrib_array(location);
  let color = ctx.get_attrib_location(&program, "color") as u32;
  ctx.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false,0,0) ;
  ctx.enable_vertex_attrib_array(color);
  ctx.draw_elements_with_i32(
    WebGlRenderingContext::TRIANGLES,
    indices.len() as i32,
    WebGlRenderingContext::UNSIGNED_SHORT,
    0,
  );
  let location = ctx.get_attrib_location(&program, "coordinates");
  Ok(location)
}
