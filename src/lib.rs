use std::vec;

use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod plotter;
use plotter::plot;

#[wasm_bindgen]
pub fn wasm_gen_coords(
    x_size: i32,
    y_size: i32,
    input_math: &str) -> Vec<i32> {
        console_error_panic_hook::set_once();
        log(&x_size.to_string());
        log(&y_size.to_string());
        log(&input_math);
        let points_op = plot(
            String::from("cyan"), 
            x_size as i64, 
            y_size as i64, 
            String::from("null"),
            false,
            false,
            true,
            String::from(input_math)
        );
        match points_op {
            Some(_) => {
                let points = points_op.unwrap();
                let mut flat: Vec<i32> = Vec::new();
                for (x, y) in points {
                    flat.push(x.round() as i32);
                    flat.push(y.round() as i32);
                }
                return flat;
            },
            None => {
                return vec![0];
            },
        }
    }

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_coord(a: f32, b: f32);
}