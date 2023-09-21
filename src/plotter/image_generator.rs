pub mod image_generator {

    use image::{RgbImage, Rgb, ImageBuffer};

    pub const WIDTH: u32 = 540;
    pub const HEIGHT: u32 = 540;

    pub fn generate_image(points: Vec<(f32, f32)>, color: &String, path: &String) {
        let mut img = generate_base_image();
        let color = get_color(color);

        // prev useless in current implementation, will be useful if optimizations are added
        let mut prev = (WIDTH, HEIGHT);
        for (x, y) in points {
            let x = x + WIDTH as f32 / 2.0;
            let mut y = HEIGHT as f32 / 2.0 - y;
            if y >= HEIGHT as f32 {
                y = HEIGHT as f32;
            } else if y <= 0.0 {
                y = 0.0;
            }
            if y > 0.0 && y < HEIGHT as f32 {
                img.put_pixel(x as u32, y as u32, color);
            }
            if prev != (WIDTH, HEIGHT) {
                let prev_y = prev.1 as f32;
                let prev_x = prev.0 as f32;
                if prev_x < x - 1.0 {
                    let mut inter_x: f32 = prev.0 as f32;
                    while inter_x < x {
                        let inter_y: f32 = prev_y - (prev_y - y) * (inter_x - prev_x) / (x - prev_x);
                        if inter_y > 0.0 && inter_y < HEIGHT as f32 { 
                            img.put_pixel(inter_x as u32, inter_y as u32, color);
                        }
                        inter_x += 1.0;
                    }
                // these will need to move if optimizations are implemented
                } else if prev_y < y {
                    for inter_y in prev_y as u32..y as u32 {
                        img.put_pixel(x as u32, inter_y, color);
                    }
                } else if prev_y > y {
                    for inter_y in y as u32..prev_y as u32 {
                        img.put_pixel(x as u32, inter_y, color);
                    }
                }
            }
            prev = (x as u32, y as u32);
        }

        // write it out to a file
        if path != "null" {
            img.save(path).unwrap();
        }
    }

    fn generate_base_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(WIDTH, HEIGHT);
        generate_axes(WIDTH / 2, WIDTH / 2, 6, &mut img);
        img
    }

    fn generate_axes(
        center: u32, mut length: u32, mut num: u32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>
    ) {
        if num > 0 {
            if num > 10 {
                num = 10;
            }
            let red = num * 25;
            let col = Rgb([red as u8, 0, 0]);
            for i in 0..HEIGHT {
                if img.get_pixel(center, i).0[0] == 0 {
                    img.put_pixel(center, i, col);
                }
            }
            for i in 0..WIDTH {
                if img.get_pixel(i, center).0[0] == 0 {
                    img.put_pixel(i, center, col);
                }
            }
            num -= 1;
            length /= 2;
            generate_axes(center - length, length, num, img);
            generate_axes(center + length, length, num, img);
        }
    }

    fn get_color(color: &String) -> Rgb<u8> {
        match color.as_str() {
            "red" => Rgb([255, 0, 0]),
            "blue" => Rgb([0, 0, 255]),
            "green" => Rgb([0, 255, 0]),
            "yellow" => Rgb([255, 255, 0]),
            "magenta" => Rgb([255, 0, 255]),
            "cyan" => Rgb([0, 255, 255]),
            "white" => Rgb([255, 255, 255]),
            "black" => Rgb([0, 0, 0]),
            _ => Rgb([255, 255, 255]),
        }
    }
}