pub mod image_generator {
    use image::{RgbImage, Rgb, ImageBuffer};

    pub const WIDTH: u32 = 1080;
    pub const HEIGHT: u32 = 1080;

    pub fn generate_image(points: Vec<(f32, f32)>, color: &String) {
        let mut img = generate_base_image();
        let color = get_color(color);

        let mut prev = (WIDTH, HEIGHT);
        for (x, y) in points {
            let x = x + 540.0;
            let mut y = 540.0 - y;
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
        img.save("images/output.png").unwrap();
    }

    fn generate_base_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(WIDTH, HEIGHT);
        generate_axes(540, 540, 6, &mut img);
        for i in 0..WIDTH {
            let col = Rgb([255, 0, 0]);
            //redraw main axes to be on top
            img.put_pixel(540, i, col);
            img.put_pixel(i, 540, col); 
        }
        img
    }

    fn generate_axes(
        center: u32, mut length: u32, mut num: u32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>
    ) {
        if num < 1 {
            return;
        } else if num > 10 {
            num = 10;
        }
        let mut red = 1;
        if length > 100 {
            red = (length/2) - 15;
        }
        else if length > 15 {
            red = length/2;
        }
        let col = Rgb([red as u8, 0, 0]);
        for i in 0..WIDTH {
            img.put_pixel(center, i, col);
            img.put_pixel(i, center, col); 
        }
        length /= 2;
        generate_axes(center + length, length, num - 1, img);
        generate_axes(center - length, length, num - 1, img);
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