use gif::Frame;
use image::{RgbImage, Rgb};
use rand::Rng;
// local import
use crate::bsc::generate_random_color;

fn random_shape(image: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    let image_width = image.dimensions().0;
    let image_height = image.dimensions().1;
    // DRAW shape
    // generate random image coordinates
    let shape_coord_x_begin = rng.gen_range(0, image_width);
    let shape_coord_x_end = rng.gen_range(shape_coord_x_begin, image_width);
    let shape_coord_y_begin = rng.gen_range(0, image_height);
    let shape_coord_y_end = rng.gen_range(shape_coord_y_begin, image_height);
    // generate shape color
    let shape_color = generate_random_color();
    // draw the new shape
    for x in shape_coord_x_begin..shape_coord_x_end {
        for y in shape_coord_y_begin..shape_coord_y_end {
            image.put_pixel(x, y, Rgb(shape_color));
        }
    }
}

pub fn animation_frames<'a>(width: u32, height: u32, n_frames: i32) -> Vec<Frame<'static>> {
    let mut frames: Vec<Frame> = Vec::new();
    let mut image = RgbImage::new(width, height);
    // OPERATION
    for _i in 1..n_frames {
        let mut image_clone = image.clone();
        random_shape(&mut image_clone);
        // makes 'image' variable be the new rgbimage
        image = image_clone.clone();
        // CREATE FRAME
        let frame = Frame::from_rgb(width as u16, height as u16, &image_clone.into_vec());
        frames.push(frame);
    }
    frames
}
