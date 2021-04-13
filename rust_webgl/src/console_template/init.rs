use crate::console_template::draw_layout::GridLayout;
use crate::console_template::draw_square::Square;
// use crate::console_template::draw_layout::init_layout;
// use crate::console_template::draw_square::init_square;
// use crate::draw_layout;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
pub struct Canvas {
  pub canvas: HtmlCanvasElement,
  pub ctx: WebGlRenderingContext,
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
    ctx.clear_color(1.0, 1.0, 1.0, 1.0);
    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    let vert_shader = compile_shader(
      &ctx,
      WebGlRenderingContext::VERTEX_SHADER,
      r#"
        attribute vec3 coordinates;
        attribute vec3 color;
        varying vec3 vColor;
        uniform vec4 translation;
        void main() {
            gl_Position = vec4(coordinates, 1.0) + translation;
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
    let translation = ctx
      .get_uniform_location(&program, "translation")
      .ok_or("failed to get uniform location")?;
    ctx.uniform4f(Some(&translation), 0.0, 0.0, 0.0, 0.0);
    let program = link_program(&ctx, &vert_shader, &frag_shader)?;
    ctx.use_program(Some(&program));
    Ok(Canvas { canvas, ctx })
  }
  pub fn update(
    &self,
    callback: wasm_bindgen::prelude::Closure<dyn std::ops::FnMut(web_sys::WheelEvent)>,
  ) -> Result<i32, JsValue>{
    &self
      .canvas
      .add_event_listener_with_callback("wheel", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(1)
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

pub fn draw_sq(
  // ctx: std::rc::Rc<web_sys::WebGlRenderingContext>,
  // canvas: &HtmlCanvasElement,
  canvas: Canvas,
  mut x: f32,
  mut y: f32,
  mut z: f32,
  scale: i32,
  init_square: Square,
) -> Result<i32, JsValue> {
  let translation = init_square.translation;
  let indices = init_square.indices;
  let ctx = canvas.ctx;
  let canvas = canvas.canvas;
  {
    let wheel_callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
      x = x - (event.delta_x() / 2000.0) as f32;
      y = y + (event.delta_y() / 2000.0) as f32;
      ctx.uniform4f(Some(&translation), x, y, z, 0.0);
      ctx.draw_elements_with_i32(
        WebGlRenderingContext::TRIANGLES,
        indices.len() as i32,
        WebGlRenderingContext::UNSIGNED_SHORT,
        0,
      );
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", wheel_callback.as_ref().unchecked_ref())?;
    wheel_callback.forget();
  }
  Ok(1)
}

pub fn draw_lay(
  // ctx: std::rc::Rc<web_sys::WebGlRenderingContext>,
  // canvas: &HtmlCanvasElement,
  canvas: Canvas,
  mut x: f32,
  mut y: f32,
  mut z: f32,
  scale: i32,
  init_layout: GridLayout,
) -> Result<i32, JsValue> {
  let translation = init_layout.translation;
  let vertices = init_layout.vertices;
  let ctx = canvas.ctx;
  let canvas = &canvas.canvas;
  {
    let wheel_callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
      x = x - (event.delta_x() / 2000.0) as f32;
      y = y + (event.delta_y() / 2000.0) as f32;
      ctx.uniform4f(Some(&translation), x, y, z, 0.0);
      ctx.draw_arrays(WebGlRenderingContext::LINES, 0, (vertices.len() / 2) as i32);
    }) as Box<dyn FnMut(_)>);
    // canvas.add_event_listener_with_callback("wheel", wheel_callback.as_ref().unchecked_ref())?;
    // wheel_callback.forget();
  }
  Ok(1)
}
