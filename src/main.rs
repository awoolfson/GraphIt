/*
TODO:
fix vertical lines with tan
debug ln and log
add more functions
debug fading line at the end of picture on web
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

    plotter::plot(color, x_size, y_size, img_path, gen_image, true, false, clean_input);
}