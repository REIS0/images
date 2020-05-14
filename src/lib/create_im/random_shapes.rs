use gif::Frame;
use image::{imageops, RgbImage};
use rand::Rng;
// local import
use crate::bsc::generate_random_color;

// TODO: debug
fn draw_shape(width: u32, height: u32) -> RgbImage {
    let color = generate_random_color();
    let image = RgbImage::new(width, height);
    image::Rgb([color.0, color.1, color.2]);
    return image;
}

fn random_shape(image: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    let image_width = image.dimensions().0;
    let image_height = image.dimensions().1;
    let new_shape = draw_shape(
        rng.gen_range(1, image_width),
        rng.gen_range(1, image_height),
    );
    // image operations
    imageops::overlay(
        image,
        &new_shape,
        rng.gen_range(0, image_width),
        rng.gen_range(0, image_height),
    );
}

pub fn animation_frames<'a>(width: u32, height: u32, n_frames: i32) -> Vec<Frame<'static>> {
    let mut frames: Vec<Frame> = Vec::new();
    let mut image = RgbImage::new(width, height);
    for _i in 0..n_frames {
        let mut image_clone = image.clone();
        random_shape(&mut image_clone);
        // makes 'image' variable be the new rgbimage
        image = image_clone.clone();
        // CREATE FRAME
        // TODO: checar panic
        let frame = Frame::from_rgb(width as u16, height as u16, &image_clone.into_vec());
        frames.push(frame);
    }
    return frames;
}
