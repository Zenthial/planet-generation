use image;
use noise::utils::NoiseImage;

pub fn write_image_to_file(image: &NoiseImage, filename: &str) {
    use std::{fs, path::Path};

    let target_dir = Path::new("example_images/");

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    let target = target_dir.join(Path::new(filename));

    let (width, height) = (1024, 1024);
    let mut result = Vec::with_capacity(width * height);

    for i in image {
        for j in i.iter() {
            result.push(*j);
        }
    }

    let _ = image::save_buffer(
        &Path::new(target.to_str().unwrap()),
        &*result,
        1024 as u32,
        1024 as u32,
        image::ColorType::Rgba8,
    );

    println!("\nFinished generating {}", filename);
}
