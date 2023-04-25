// each horizontal represents one print on the graph, so one y value
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
    // x_window and y_window are the ranges of the graph, and the size of the graph, will take in as arguments later
    let x_window = (-25, 25, 50); // lower, upper, size
    let y_window = (-25, 25, 50);
    let length = y_window.2;
    let mut lines = Vec::new();
    for _ in 0..length {
        lines.push(Horizontal {
            x_window: x_window,
            points: Vec::new(),
            is0: false,
        });
    }
    for x_val in x_window.0..x_window.1 {
        let height = math(x_val);
        if height > y_window.0 && height < y_window.1 {
            let y_index = height_to_index(height, y_window);
            lines[y_index as usize].points.push((x_val - x_window.0).try_into().unwrap());
        }
    }
    lines[length as usize / 2].is0 = true;
    lines.iter().for_each(|x| x.print());
}

// math is the function that is being graphed, will code parser later
fn math(x: i64) ->  i64 {
    let output = x + x;
    output
}

// used to convert the height to the index of the horizontal
fn height_to_index(height: i64, y_window: (i64, i64, i64)) -> i64 {
    let inverted = height - y_window.0;
    y_window.2 - inverted
}