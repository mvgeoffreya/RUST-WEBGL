use crate::console_template::draw_layout::draw_layout;
use crate::console_template::draw_square::draw_square;
use crate::Payload;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use super::draw_layout::ImageLayout;
// use super::draw_square::Square;
#[derive(Debug,Clone)]
pub struct Canvas {
  pub canvas: HtmlCanvasElement,
  pub ctx: WebGlRenderingContext,
  pub program: web_sys::WebGlProgram,
  pub translation: web_sys::WebGlUniformLocation,
  pub location: u32,
  pub color: u32,
}

impl Canvas {
  pub fn new(attr_id: &str) -> Result<Self, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(attr_id).unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let ctx = canvas
      .get_context("webgl")?
      .unwrap()
      .dyn_into::<WebGlRenderingContext>()?;
    // let vert_shader = compile_shader(
    //   &ctx,
    //   WebGlRenderingContext::VERTEX_SHADER,
    //   r#"
    //       attribute vec3 coordinates;
    //       attribute vec3 color;
    //       varying vec3 vColor;
    //       uniform vec4 translation;
    //       void main() {
    //           gl_Position = vec4(coordinates, 1.0) + translation;
    //           vColor = color;
    //       }
    //   "#,
    // )?;
    // let frag_shader = compile_shader(
    //   &ctx,
    //   WebGlRenderingContext::FRAGMENT_SHADER,
    //   r#"
    //       precision mediump float;
    //       varying vec3 vColor;
    //       void main() {
    //         gl_FragColor = vec4(vColor, 1.);
    //       }
    //   "#,
    // )?;

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
    let location: u32 = ctx.get_attrib_location(&program, "coordinates") as u32;
    let color = ctx.get_attrib_location(&program, "color") as u32;
    let translation = ctx
      .get_uniform_location(&program, "translation")
      .ok_or("failed to get uniform location")?;
    ctx.clear_color(1.0, 1.0, 1.0, 1.0);
    ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    Ok(Self {
      canvas,
      ctx,
      program,
      translation,
      location,
      color,
    })
  }

  pub fn draw(
    &self,
    layout: &ImageLayout,
    square: &Payload,
    x: f32,
    y: f32,
    z: f32,
  ) -> Result<i32, JsValue> {
    let ctx = &self.ctx;
    let translation = &self.translation;
    let color = self.color;
    let vertices_layout = &layout.vertices;
    let vertices_square = &square.vertices;
    let colors_square = &square.colors;
    let indices_square = &square.indices;
    let _draw_layout = draw_layout(ctx, translation, vertices_layout, x, y, z);
    let _draw_square = draw_square(
      ctx,
      translation,
      vertices_square,
      x,
      y,
      z,
      colors_square,
      indices_square,
      color,
    );
    Ok(3)
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
