// each horizontal represents one print on the graph, so one y value
use::std::env;
use std::io::*;
mod parser;
pub use parser::*;
struct Horizontal {
    x_window: (i64, i64, usize),
    points: Vec<usize>,
    is0: bool,
}

impl Horizontal {
    // print constructs the string and prints it
    fn print(&self) {
        let mut line: String;
        if self.is0 {
            line = std::iter::repeat("-").take(self.x_window.2).collect::<String>();
        } else {
            line = std::iter::repeat(" ").take(self.x_window.2).collect::<String>();
        }
        Self::replace_char(&mut line, self.x_window.2 / 2, '|');
        for x in self.points.iter() {
            Self::replace_char(&mut line, *x, '*');
        }
        println!("{}", line);
    }

    // replace_char replaces the character at index with newchar, used for plotting
    fn replace_char(s: &mut str, index: usize, newchar: char) {
        let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
        assert!(index < s_bytes.len());
        assert!(s_bytes[index].is_ascii());
        assert!(newchar.is_ascii());
        s_bytes[index] = newchar as u8;
    }
}

fn main() {
    let debug = false;
    if debug {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let mut x_size: i64 = 32;
    let mut y_size: i64 = 32;

    let args: Vec<String> = env::args().collect();
    for (idx, a) in args.iter().enumerate() {
        if a == "-xsize" {
            x_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-ysize" {
            y_size = args[idx + 1].parse::<i64>().unwrap();
        }
    }

    println!("f(x) = ...");
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string);
    let clean_input = String::from(input_string.trim()); 

    let tokens = parser::parser::tokenize(clean_input).unwrap();

    if debug { 
        println!("\ninitial tokens\n");
        for t in &tokens {
            match t {
                parser::parser::Token::Num(n) => println!("{}", n),
                parser::parser::Token::Operator(o) => println!("{:?}", o),
                parser::parser::Token::Var => println!("x"),
            }
        }
    }

    let postfix = parser::parser::infix_to_postfix(tokens);

    if debug {
        println!("postfix tokens\n");
        for t in &postfix {
            match t {
                parser::parser::Token::Num(n) => println!("{}", n),
                parser::parser::Token::Operator(o) => println!("{:?}", o),
                parser::parser::Token::Var => println!("x"),
            }
        }
    }

    let x_window;
    let y_window;

    //args

    x_window = (-32, 32, 64);
    y_window = (-16, 16, 32);
    let increment_parts = 1; // how many parts to divide each 1 by for iteration along x axis, 1 is the minimum

    let x_normalizer: f32 = x_window.2 as f32 / x_size as f32; // for conversion from real coords to math coords (/)
    let y_normalizer: f32 = y_window.2 as f32 / y_size as f32; // for conversion from math coords to real coords (*)
    if debug { println!("x normalizer: {}, y_normalizer: {}", x_normalizer, y_normalizer); }

    let mut lines = Vec::new();
    for _ in 0..y_window.2 {
        lines.push(Horizontal {
            x_window: x_window,
            points: Vec::new(),
            is0: false,
        });
    }

    for x_val in x_window.0..x_window.1 {
        // consider removing the increment feature, maybe this is silly
        let normalized_x = x_val as f32 / x_normalizer;
        for i in 0..increment_parts {
            let incremented_x = normalized_x + (i as f32 / (x_size as f32)/increment_parts as f32);
            let raw_height = math_on_postfix(&postfix, incremented_x);
    
            if debug { println!("x: {}, normalized_x: {}, raw height: {}", x_val, normalized_x, raw_height); }
    
            let normalized_y = raw_height * y_normalizer;
    
            if normalized_y > y_window.0 as f32 && normalized_y < y_window.1 as f32 {
                let horizontals_index = height_to_index(normalized_y, y_window);
                if debug { println!("normalized y: {}, y index: {}\n", normalized_y, horizontals_index); }
                lines[horizontals_index as usize].points.push((x_val - x_window.0).try_into().unwrap());
            } else {
                if debug { println!("y out of range\n"); }
            }
        }
    }

    lines[y_window.2 as usize / 2].is0 = true;
    lines.iter().for_each(|x| x.print());
}

// math is the function that is being graphed, will code parser later
// fn math(x: f32) ->  f32 {
//     let output = x.sin();
//     output
// }

fn math_on_postfix(postfix: &Vec<parser::parser::Token>, x: f32) -> f32 {
    let mut stack: Vec<f32> = Vec::new();
    for t in postfix {
        match t {
            parser::parser::Token::Num(n) => stack.push(*n),
            parser::parser::Token::Var => stack.push(x),
            parser::parser::Token::Operator(o) => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                match o {
                    parser::parser::Operator::Exp(_) => stack.push(num2.powf(num1)),
                    parser::parser::Operator::Multiply(_) => stack.push(num2 * num1),
                    parser::parser::Operator::Divide(_) => stack.push(num2 / num1),
                    parser::parser::Operator::Add(_) => stack.push(num2 + num1),
                    parser::parser::Operator::Subtract(_) => stack.push(num2 - num1),
                    _=> {},
                }
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