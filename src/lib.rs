use wasm_bindgen::prelude::*;
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
                return vec![0];
            },
        }
    }