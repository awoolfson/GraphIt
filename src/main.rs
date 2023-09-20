//TODO: fix graph lines using crosses instead of lines, fix filepath for image generation
use::std::env;
use::std::path::Path;
mod plotter;

pub fn wasm_gen_image(
    color: &str, 
    x_size: isize, 
    y_size: isize,
    input_math: &str) {
        plotter::plot(
            String::from(color), 
            x_size as i64, 
            y_size as i64, 
            String::from("images/output.png"),
            true, 
            false, 
            String::from(input_math)
        );
    }

fn main() {

    let mut x_size: i64 = 32;
    let mut y_size: i64 = 32;

    let mut img_path = String::from("images/output.png");
    let mut gen_image = false;

    let mut color: String = String::new();

    let mut args: Vec<String> = env::args().collect();
    args.push("null".to_string());
    for (idx, a) in args.iter().enumerate() {
        if a == "-xsize" {
            x_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-ysize" {
            y_size = args[idx + 1].parse::<i64>().unwrap();
        } else if a == "-c" {
            color = args[idx + 1].clone();
        } else if a == "-i" {
            img_path = args[idx + 1].clone();
            gen_image = true;
            //currently doesn't work, need path from root? just don't put path for now
            if !Path::new(&img_path).exists() {
                img_path = String::from("images/output.png")
            }
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
    //wasm_gen_image(&color, x_size as isize, y_size as isize, &clean_input);
}