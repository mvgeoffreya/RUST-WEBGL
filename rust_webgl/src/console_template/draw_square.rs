use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
pub struct Square {
  pub vertices: Vec<f32>,
  pub colors: Vec<f32>,
  pub indices: Vec<u16>,
}

impl Square {
  pub fn init_square(
    ctx: &WebGlRenderingContext,
    translation: &web_sys::WebGlUniformLocation,
    color: u32,
    scale: i32,
    x: f32,
    y: f32,
    z: f32,
  ) -> Result<Square, JsValue> {
    let colors: Vec<f32> = [0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0, 0.1, 0.0, 0.1].to_vec();
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
    let vertices: Vec<f32> = [
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

    let vertex_buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    unsafe {
      let vert_array = js_sys::Float32Array::view(&vertices);
      ctx.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGlRenderingContext::STATIC_DRAW,
      );
      ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    }
    // vertex
    // index
    let indices: Vec<u16> = [3, 2, 1, 3, 1, 0].to_vec();
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
      ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }

    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    ctx.bind_buffer(
      WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
      Some(&index_buffer),
    );
    ctx.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    ctx.enable_vertex_attrib_array(0);
    ctx.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    ctx.enable_vertex_attrib_array(color);
    ctx.uniform4f(Some(&translation), x, y, z, 0.0);
    ctx.draw_elements_with_i32(
      WebGlRenderingContext::TRIANGLES,
      indices.len() as i32,
      WebGlRenderingContext::UNSIGNED_SHORT,
      0,
    );
    Ok(Square {
      vertices,
      colors,
      indices,
    })
  }
}

pub fn draw_square(ctx: &web_sys::WebGlRenderingContext,translation: &web_sys::WebGlUniformLocation, vertices: &Vec<f32>, x: f32, y: f32, z: f32, colors: &Vec<f32>,  indices: &Vec<u16>, color: u32 ) -> Result<i32,JsValue> {

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
  let vertex_buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
  unsafe {
    let vert_array = js_sys::Float32Array::view(&vertices);
    ctx.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &vert_array,
      WebGlRenderingContext::STATIC_DRAW,
    );
    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
  }

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
    ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
  }

  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
  ctx.bind_buffer(
    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
    Some(&index_buffer),
  );
  ctx.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
  ctx.enable_vertex_attrib_array(0);
  ctx.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
  ctx.enable_vertex_attrib_array(color);


  ctx.uniform4f(Some(&translation), x, y, z, 0.0);
  ctx.draw_elements_with_i32(
    WebGlRenderingContext::TRIANGLES,
    indices.len() as i32,
    WebGlRenderingContext::UNSIGNED_SHORT,
    0,
  );
  Ok(1)
}