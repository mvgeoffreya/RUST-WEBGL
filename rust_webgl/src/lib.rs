extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
// mod console_template;
// use console_template::draw::Canvas;

// #[wasm_bindgen]
// pub fn init(att_id: &str, width: i32, times: i32, height: i32, scale: i32) {
//     let canvas = Canvas::new(att_id);
//     canvas.clear_layout(width, height, scale);
//     canvas.prepare_layout(width, times, scale);
// }
// let document = web_sys::window().unwrap().document().unwrap();
// let canvas = document.get_element_by_id("canvas").unwrap();
// let canvas: web_sys::HtmlCanvasElement = canvas
//     .dyn_into::<web_sys::HtmlCanvasElement>()
//     .map_err(|_| ())
//     .unwrap();

// let context = canvas
//     .get_context("2d")
//     .unwrap()
//     .unwrap()
//     .dyn_into::<web_sys::CanvasRenderingContext2d>()
//     .unwrap();

// context.begin_path();

// // Draw the outer circle.
// context
//     .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
//     .unwrap();

// // Draw the mouth.
// context.move_to(110.0, 75.0);
// context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

// // Draw the left eye.
// context.move_to(65.0, 65.0);
// context
//     .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
//     .unwrap();

// // Draw the right eye.
// context.move_to(95.0, 65.0);
// context
//     .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
//     .unwrap();

// context.stroke();

#[wasm_bindgen]

pub fn start(scale: i32) -> Result<Vec<f32>, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("init").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));
    let mut vertices = Vec::new();
 
    for points in 0..200 {
        vertices.push(-1.0* scale as f32);
        vertices.push(((points-100)as f32 / 100.0) * scale as f32);
        vertices.push(((100-points) as f32 / 100.0) * scale as f32);
        vertices.push(1.0* scale as f32);

        vertices.push(((points-100)as f32 / 100.0) * scale as f32);
        vertices.push(-1.0* scale as f32);
        vertices.push(1.0* scale as f32);
        vertices.push(((100-points) as f32 / 100.0) * scale as f32);
        
        
        vertices.push(((points-100)as f32 / 100.0) * scale as f32);
        vertices.push(1.0* scale as f32);
        vertices.push(1.0* scale as f32);
        vertices.push(((points-100) as f32 / 100.0) * scale as f32);

        vertices.push(((-points+100)as f32 / 100.0) * scale as f32);
        vertices.push(-1.0* scale as f32);
        vertices.push(-1.0* scale as f32);
        vertices.push(((-points+100) as f32 / 100.0) * scale as f32);
    }


    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::LINES,
        0,
        (vertices.len() / 2) as i32,
    );
    Ok(vertices)
    
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
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
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
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
