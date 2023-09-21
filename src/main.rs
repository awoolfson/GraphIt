/*
TODO:
1. figure out why y values are registered as infinity and fix
2. remove all path options and just set it as a constant in main
3. modularize coord generation so that it can be called from wasm without making an image
maybe a 3rd bool argument (gen coords?)
4. edit base and base function for max red value instead of first
5. make base drawing function seperate from everything else, only happens
if base image not available, will need to load png into rust image for this
6. also maybe set image size as constant 540x540, it looks nice
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

    plotter::plot(color, x_size, y_size, img_path, gen_image, true, clean_input);
}