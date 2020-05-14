use gif::Frame;
use image::{Rgb, RgbImage};
// local import
use crate::bsc::generate_random_color;

// TODO: make function work
fn draw_shape(width: u32, height: u32, image: &mut RgbImage) {
    let color = generate_random_color();
    for y in 0..width {
        for x in 0..height {
            image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
        }
    }
}

pub fn animation_frames<'a>(width: u32, height: u32, n_frames: i32) -> Vec<Frame<'static>> {
    let mut frames: Vec<Frame> = Vec::new();
    let mut image = RgbImage::new(width, height);
    // set the variables to use during the operation
    let mut width = width;
    let mut height = height;
    let mut frame_to_divide = n_frames / 2;
    for _i in 0..n_frames {
        let mut image_clone = image.clone();
        draw_shape(width, height, &mut image_clone);
        // makes 'image' variable be the new rgbimage
        image = image_clone.clone();
        // CREATE FRAME
        // TODO: checar panic
        let frame = Frame::from_rgb(width as u16, height as u16, &image_clone.into_vec());
        frames.push(frame);
        // divide to create a smaller shape
        width = width / frame_to_divide as u32;
        height = width / frame_to_divide as u32;
        frame_to_divide = frame_to_divide / 2;
        if width < 1 || height < 1 {
            break;
        }
    }
    return frames;
}
