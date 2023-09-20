pub mod parser;
pub mod horizontal;
pub mod image_generator;

use parser::parser as p;
use horizontal::horizontal as h;
use image_generator::image_generator as img;

pub fn plot(
    color: String, 
    x_size: i64, 
    y_size: i64,
    img_path: String,
    gen_image: bool, 
    print_cli: bool,
    input_math: String) -> Option<Vec<u8>> {

    println!("plotting");

    let tokens = p::tokenize(input_math).unwrap();
    let postfix = p::infix_to_postfix(tokens);

    if print_cli {
        let x_window = (-32, 32, 64); // (min, max, size)
        let y_window = (-16, 16, 32);
        let x_normalizer_cli: f32 = x_window.2 as f32 / x_size as f32;
        let y_normalizer_cli: f32 = y_window.2 as f32 / y_size as f32;

        let mut lines = Vec::new();
        for _ in 0..y_window.2 {
            lines.push(h::Horizontal {
                x_window: (x_window.0, x_window.1, x_window.2 as usize),
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
    }

    if gen_image {
        let x_normalizer_img: f32 = img::WIDTH as f32 / x_size as f32;
        let y_normalizer_img: f32 = img::HEIGHT as f32 / y_size as f32;

        let mut points = Vec::<(f32, f32)>::new();
        
        let lower = -(img::WIDTH as i32)/2;
        let upper = (img::WIDTH as i32)/2;
        for x_val in lower..upper {
            let normalized_x = x_val as f32 / x_normalizer_img;
            let raw_height = math_on_postfix(&postfix, normalized_x as f32);
            let normalized_y = raw_height * y_normalizer_img;
            points.push((x_val as f32, normalized_y)); 
        }
        let buf = img::generate_image(points, &color, &img_path);
        return Some(buf);
    }
    None
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