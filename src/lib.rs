use std::vec;

use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod plotter;
use plotter::plot;

#[wasm_bindgen]
pub fn wasm_gen_coords(
    color: &str, 
    x_size: isize, 
    y_size: isize,
    input_math: &str) -> Vec<i32> {
        console_error_panic_hook::set_once();
        let points_op = plot(
            String::from(color), 
            x_size as i64, 
            y_size as i64, 
            String::from("null"),
            true, 
            false, 
            String::from(input_math)
        );
        match points_op {
            Some(_) => {
                let points = points_op.unwrap();
                let mut flat: Vec<i32> = Vec::new();
                for (x, y) in points {
                    flat.push(x as i32);
                    flat.push(y as i32);
                }
                return flat;
            },
            None => {
                return vec![0];
            },
        }
    }