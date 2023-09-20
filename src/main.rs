/*
TODO:
1. try changing all fourth values to 255 instead of 1 (for alpha)
2. if that doesn't work, add base image as png to directory and link it from javascript, then return points
from wasm and manually draw them on canvas
3. make local command always add image to images foler as output.png, remove any option for filepath
 */
use::std::env;
mod plotter;

fn main() {

    let mut x_size: i64 = 32;
    let mut y_size: i64 = 32;

    let img_path = String::from("images/output.png");
    let mut gen_image = false;

    let mut color: String = String::new();

    let mut args: Vec<String> = env::args().collect();
    args.push("nil".to_string());
    for (idx, a) in args.iter().enumerate() {
        if a == "-xsize" {
            x_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-ysize" {
            y_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-c" {
            color = args[idx + 1].clone();
        } else if a == "-i" {
            gen_image = true;
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

    plotter::plot(color, x_size, y_size,  img_path, gen_image, true, clean_input);
}