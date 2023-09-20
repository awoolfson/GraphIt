pub mod horizontal {
    use::colored::Colorize;

    pub struct Horizontal {
        pub x_window: (i64, i64, usize),
        pub points: Vec<usize>,
        pub is0: bool,
    }

    impl Horizontal {
        // print constructs the string and prints it
        pub fn print(&self, color: &String) {
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
            println!("{}", Self::color(&mut line, color));
        }

        // replace_char replaces the character at index with newchar, used for plotting
        fn replace_char(s: &mut str, index: usize, newchar: char) {
            let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
            assert!(index < s_bytes.len());
            assert!(s_bytes[index].is_ascii());
            assert!(newchar.is_ascii());
            s_bytes[index] = newchar as u8;
        }

        fn color(input: &mut String, color: &String) -> String {
            match color.as_str() {
                "red" => input.red().to_string(),
                "blue" => input.blue().to_string(),
                "green" => input.green().to_string(),
                "yellow" => input.yellow().to_string(),
                "magenta" => input.magenta().to_string(),
                "cyan" => input.cyan().to_string(),
                "white" => input.white().to_string(),
                "black" => input.black().to_string(),
                _ => input.to_string(),
            }
        }
    }
}