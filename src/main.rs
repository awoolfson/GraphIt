// each horizontal represents one print on the graph, so one y value
use::std::env;
mod parser;
pub use parser::parser as p;
mod image_generator;
pub use image_generator::image_generator as img;

use crate::img::generate_image;
mod horizontal;

fn main() {

    let mut x_size: i64 = 32;
    let mut y_size: i64 = 32;
    let mut color: String = String::new();

    let args: Vec<String> = env::args().collect();
    for (idx, a) in args.iter().enumerate() {
        if a == "-xsize" {
            x_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-ysize" {
            y_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-c" {
            color = args[idx + 1].clone();
        }
    }

    println!("f(x) = ...");
    let mut input_string = String::new();
    let res = std::io::stdin().read_line(&mut input_string);
    if let Err(_) = res {
        println!("error reading input");
        return;
    }
    let clean_input = String::from(input_string.trim()); 
    let tokens = p::tokenize(clean_input).unwrap();
    let postfix = p::infix_to_postfix(tokens);

    let x_window = (-32, 32, 64); // (min, max, size)
    let y_window = (-16, 16, 32);

    let x_normalizer_cli: f32 = x_window.2 as f32 / x_size as f32;
    let y_normalizer_cli: f32 = y_window.2 as f32 / y_size as f32;
    let x_normalizer_img: f32 = img::WIDTH as f32 / x_size as f32;
    let y_normalizer_img: f32 = img::HEIGHT as f32 / y_size as f32;

    let mut lines = Vec::new();
    for _ in 0..y_window.2 {
        lines.push(horizontal::Horizontal {
            x_window: x_window,
            points: Vec::new(),
            is0: false,
        });
    }

    for x_val in x_window.0..x_window.1 {
        // for cli
        let normalized_x = x_val as f32 / x_normalizer_cli;
        let raw_height = math_on_postfix(&postfix, normalized_x);
        let normalized_y = raw_height * y_normalizer_cli;
        if normalized_y > y_window.0 as f32 && normalized_y < y_window.1 as f32 {
            let horizontals_index = height_to_index(normalized_y, y_window);
            lines[horizontals_index as usize].points.push((x_val - x_window.0).try_into().unwrap());
        }
    }

    lines[y_window.2 as usize / 2].is0 = true;
    lines.iter().for_each(|x| x.print(&color));

    let mut points = Vec::<(f32, f32)>::new();
    
    let lower = -(img::WIDTH as i32)/2;
    let upper = (img::WIDTH as i32)/2;
    for x_val in lower..upper {
        let normalized_x = x_val as f32 / x_normalizer_img;
        let raw_height = math_on_postfix(&postfix, normalized_x as f32);
        let normalized_y = raw_height * y_normalizer_img;
        points.push((x_val as f32, normalized_y)); 
    }

    generate_image(points);
}

fn math_on_postfix(postfix: &Vec<p::Token>, x: f32) -> f32 {
    let mut stack: Vec<f32> = Vec::new();
    for t in postfix {
        match t {
            p::Token::Num(n) => stack.push(*n),
            p::Token::Var => stack.push(x),
            p::Token::Operator(o) => {
                let num1 = stack.pop().unwrap();
                let mut num2: f32 = 0.0; // placeholder value
                match o {
                    p::Operator::Func(_) => {},
                    _=> { num2 = stack.pop().unwrap(); }
                }
                let y = o.execute(num2, num1);
                stack.push(y);
            }
        }
    }
    stack[0]
}

// used to convert the height to the index of the horizontal
fn height_to_index(height: f32, y_window: (i64, i64, i64)) -> i64 {
    let y_window_1 = y_window.1 as f32;
    (y_window_1 - height) as i64
}