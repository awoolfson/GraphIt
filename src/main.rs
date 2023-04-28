// each horizontal represents one print on the graph, so one y value
use::std::env;
use colored::Colorize;
mod parser;

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
        // we've made sure this is safe.
        s_bytes[index] = newchar as u8;
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //let realist = false; // if true the graph will be accurate to character spacing
    // x_window and y_window are the ranges of the graph, and the size of the graph, will take in as arguments later
    let x_window;
    let y_window;
    // if realist {
    //     x_window = (-32, 32, 64); // lower, upper, size
    //     y_window = (-32, 32, 64);
    // } else {
    //     x_window = (-32, 32, 64);
    //     y_window = (-16, 16, 32);
    // }

    //args

    x_window = (-32, 32, 64);
    y_window = (-16, 16, 32);
    let increment_parts = 1; // how many parts to divide each 1 by for iteration along x axis, 1 is the minimum
    let x_size = 20; // scope of equation, not actual size on screen
    let y_size = 4;

    let x_normalizer: f32 = x_window.2 as f32 / x_size as f32; // for conversion from real coords to math coords (/)
    let y_normalizer: f32 = y_window.2 as f32 / y_size as f32; // for conversion from math coords to real coords (*)
    println!("x normalizer: {}, y_normalizer: {}", x_normalizer, y_normalizer);

    let mut lines = Vec::new();
    for _ in 0..y_window.2 {
        lines.push(Horizontal {
            x_window: x_window,
            points: Vec::new(),
            is0: false,
        });
    }

    for x_val in x_window.0..x_window.1 {
        let normalized_x = x_val as f32 / x_normalizer;
        for i in 0..increment_parts {
            let incremented_x = normalized_x + (i as f32 / (x_size as f32)/increment_parts as f32);
            let raw_height = math(incremented_x);
    
            println!("x: {}, normalized_x: {}, raw height: {}", x_val, normalized_x, raw_height);
    
            // if !realist {
            //     raw_height /= 2.0;
            // }
    
            let normalized_y = raw_height * y_normalizer;
    
            if normalized_y > y_window.0 as f32 && normalized_y < y_window.1 as f32 {
                let horizontals_index = height_to_index(normalized_y, y_window);
                println!("normalized y: {}, y index: {}\n", normalized_y, horizontals_index);
                lines[horizontals_index as usize].points.push((x_val - x_window.0).try_into().unwrap());
            } else {
                println!("y out of range\n");
            }
        }
    }

    lines[y_window.2 as usize / 2].is0 = true;

    /*
    an issue with the following loops is that they can create "clusters", where there is a square of 4 points.
    this happens because it cannot tell if the points are next to eachother and it will still clone, however the
    problem of finding which point to clone is very difficult, so this should be fine for now.

    also doesn't work with parabolo, sends a straight line out!

    example of cluster:
    x * x * x with size 50

    This could potentially be fixed by actually making a more precise graph, incementing by 0.025 instead of 1 and then rounding
     */

    // for i in 0..lines.len() - 1 {
    //     while !lines[i].points.is_empty() && lines[i + 1].points.is_empty() {
    //         lines[i + 1].points = lines[i].points.clone();
    //     }
    // }
    // for i in (0..lines.len()).rev() {
    //     while lines[i].points.is_empty() && !lines[i + 1].points.is_empty() {
    //         lines[i].points = lines[i + 1].points.clone();
    //     }
    // }
    lines.iter().for_each(|x| x.print());
}

// math is the function that is being graphed, will code parser later
fn math(x: f32) ->  f32 {
    let output = x.sin();
    output
}

// used to convert the height to the index of the horizontal
fn height_to_index(height: f32, y_window: (i64, i64, i64)) -> i64 {
    let y_window_1 = y_window.1 as f32;
    (y_window_1 - height) as i64
}