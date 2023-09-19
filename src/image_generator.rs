pub mod image_generator {
    use image::{RgbImage, Rgb};

    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 1080;

    pub fn generate_image(points: Vec<(f32, f32)>) {
        let mut img = RgbImage::new(WIDTH, HEIGHT);

        for x in 0..=WIDTH - 1 {
            img.put_pixel(x, 540, Rgb([255, 0, 0]));
        }

        for y in 0..=HEIGHT - 1 {
            img.put_pixel(540, y, Rgb([255, 0, 0]));
        }

        let mut prev = (WIDTH, HEIGHT);
        for (x, y) in points {
            let x = x + 540.0;
            let y = 540.0 - y;
            img.put_pixel(x as u32, y as u32, Rgb([0, 255, 0]));
            if prev != (WIDTH, HEIGHT) {
                let mut inter_x: f32 = prev.0 as f32;
                while inter_x < x {
                    let prev1 = prev.1 as f32;
                    let prev0 = prev.0 as f32;
                    let inter_y: f32 = prev1 - (prev1 - y) * (inter_x - prev0) / (x - prev0);
                    img.put_pixel(inter_x as u32, inter_y as u32, Rgb([0, 255, 0]));
                    inter_x += 1.0;
                }
            }
            prev = (x as u32, y as u32);
        }

        // write it out to a file
        img.save("output.png").unwrap();
    }
}