use image::{Rgb, RgbImage};

pub fn generate_image(data: &[f32], window_size: &usize, out_path: &str, n: &usize) {
    let width = *window_size / *n;
    let height = data.len() / *window_size;

    let mut img = RgbImage::new(width as u32, height as u32);

    data.iter()
        .step_by(*n)
        .enumerate()
        .for_each(|(mut i, value)| {
            i *= n;

            let x = i % *window_size;
            let y = i / 2048;

            let clamped_value = ((value + 1.0) * 128.0).floor() as u8;

            let pixel = Rgb([clamped_value, clamped_value, clamped_value]);

            img.put_pixel((x / *n) as u32, y as u32, pixel);
        });

    if let Err(err) = img.save(out_path) {
        println!("{:?}", err);
    }
}
