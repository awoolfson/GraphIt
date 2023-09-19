pub mod image_generator {
    use image::{RgbImage, Rgb, ImageBuffer};

    pub const WIDTH: u32 = 1080;
    pub const HEIGHT: u32 = 1080;

    pub fn generate_image(points: Vec<(f32, f32)>) {
        let mut img = generate_base_image();

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
                img.put_pixel(x as u32, y as u32, Rgb([0, 255, 0]));
            }
            if prev != (WIDTH, HEIGHT) {
                let mut inter_x: f32 = prev.0 as f32;
                while inter_x < x {
                    let prev1 = prev.1 as f32;
                    let prev0 = prev.0 as f32;
                    let inter_y: f32 = prev1 - (prev1 - y) * (inter_x - prev0) / (x - prev0);
                    if inter_y > 0.0 && inter_y < HEIGHT as f32 { 
                        img.put_pixel(inter_x as u32, inter_y as u32, Rgb([0, 255, 0]));
                    }
                    inter_x += 1.0;
                }
            }
            prev = (x as u32, y as u32);
        }

        // write it out to a file
        img.save("output.png").unwrap();
    }

    fn generate_base_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(WIDTH, HEIGHT);
        for i in 0..WIDTH {
            img.put_pixel(i, 540, Rgb([255, 0, 0]));
            img.put_pixel(i, 270, Rgb([200, 0, 0]));
            img.put_pixel(i, 810, Rgb([200, 0, 0]));

            img.put_pixel(540, i, Rgb([255, 0, 0]));
            img.put_pixel(270, i, Rgb([200, 0, 0]));
            img.put_pixel(810, i, Rgb([200, 0, 0]));        
        }
        img
    }

    // fn generate_axis_coods() -> Vec<u32> {
        
    // }
}