use wasm_bindgen::{prelude::*, JsValue};
extern crate console_error_panic_hook;
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod plotter;
use plotter::plot;

#[wasm_bindgen]
pub fn wasm_gen_image(
    color: &str, 
    x_size: isize, 
    y_size: isize,
    input_math: &str) -> Vec<u8> {
        console_error_panic_hook::set_once();
        let buf_op = plot(
            String::from(color), 
            x_size as i64, 
            y_size as i64, 
            String::from("null"),
            true, 
            false, 
            String::from(input_math)
        );
        match buf_op {
            Some(_) => {
                let buf = buf_op.unwrap();
                return buf;
            },
            None => {
                return vec![];
            },
        }
    }

// #[wasm_bindgen]
// pub fn draw_on_canvas(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
//     let context = canvas
//         .get_context("2d")?
//         .unwrap()
//         .dyn_into::<CanvasRenderingContext2d>()?;

//     // Example: Draw a red rectangle
//     context.set_fill_style(&"red".into());
//     context.fill_rect(50.0, 50.0, 100.0, 100.0);

//     Ok(())
// }