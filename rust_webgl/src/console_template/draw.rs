use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
pub struct Canvas {
  pub canvas: HtmlCanvasElement,
  pub ctx: WebGlRenderingContext,
  pub program: WebGlProgram,
}

impl Canvas {
  pub fn new(attr_id: &str) -> Result<Canvas, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(attr_id).unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let ctx = canvas
      .get_context("webgl")?
      .unwrap()
      .dyn_into::<WebGlRenderingContext>()?;
    
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
    let location:u32 = ctx.get_attrib_location(&program, "coordinates") as u32;
    ctx.vertex_attrib_pointer_with_i32(location, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    ctx.enable_vertex_attrib_array(0);

    ctx.clear_color(1.0, 1.0, 1.0, 1.0);
    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    Ok(Canvas { canvas, ctx,program })
  }
}

pub fn compile_shader(
  context: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
    .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGlRenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
    .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}

pub fn draw_layout(canvas: Canvas, scale: i32) -> Result<i32, JsValue> {
  let mut vertices = Vec::new();
  let ctx: WebGlRenderingContext = canvas.ctx;
  let program: WebGlProgram = canvas.program;
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
  ctx.draw_arrays(WebGlRenderingContext::LINES, 0, (vertices.len() / 2) as i32);
  let location = ctx.get_attrib_location(&program, "coordinates");
  Ok(location)
}