use std::f64;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
pub struct Canvas {
  pub canvas: HtmlCanvasElement,
  pub ctx: CanvasRenderingContext2d,
  // scaled_width: u32,
  // scaled_height: u32,
  // width: u32,
  // height: u32,
}

impl Canvas {
  pub fn new(attr_id: &str) -> Canvas {
    let document_canvas = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .get_element_by_id(attr_id)
      .unwrap();
    let canvas = document_canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();
    let ctx = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    // let scaled_width = canvas.width() / width;
    // let scaled_height = canvas.height() / height;
    ctx.begin_path();
    Canvas {
      canvas,
      ctx,
      // scaled_width,
      // scaled_height,
      // width,
      // height,
    }
  }
  pub fn clear_layout(&self, width: i32, height: i32, scale: i32) {
    self.ctx.clear_rect(0.0,0.0,height.into(), width.into());
  }

  pub fn prepare_layout(&self, width: i32, height: i32, scale: i32) {
    for cordinate_y in 0..height {
      for cordinate_x in 0..width {
        self.ctx.move_to(
          (cordinate_x as f64) * scale as f64 * 50.0,
          (cordinate_y as f64 * scale as f64 * 250.0) + 0.0,
        );
        self.ctx.line_to(
          ((cordinate_x + 10) as f64) * scale as f64 * 50.0,
          (cordinate_y as f64 * scale as f64 * 250.0) + 250.0 * scale as f64,
        );
        self.ctx.move_to(
          ((-cordinate_x - 1) as f64) * scale as f64 * 50.0,
          (cordinate_y as f64 * scale as f64 * 250.0) + 0.0,
        );
        self.ctx.line_to(
          ((-cordinate_x - 1 + 10) as f64) * scale as f64 * 50.0,
          (cordinate_y as f64 * scale as f64 * 250.0) + 250.0 * scale as f64,
        );
      }
      for cordinate_x in 0..width {
        self.ctx.move_to(
          (cordinate_x as f64) * scale as f64 * 50.0,
          (cordinate_y as f64 * scale as f64 * 250.0) + 0.0,
        );
        self.ctx.line_to(
          ((cordinate_x - 10) as f64) * scale as f64 * 50.0,
          (cordinate_y as f64 * scale as f64 * 250.0) + 250.0 * scale as f64,
        );
      }
    }
    self.ctx.stroke();
  }
}
