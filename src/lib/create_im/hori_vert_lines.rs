use gif::Frame;
use image::{Rgb, RgbImage};
// local import
use crate::bsc::generate_random_color;

// TODO: function to return image buffer
fn image_vec(width: u32, height: u32) -> Vec<u8> {
    // CREATE IMAGE
    let mut image = RgbImage::new(width, height);
    // horizontal lines
    for y in 0..(height / 2) {
        let color = generate_random_color();
        for x in 0..(width / 2) {
            image.put_pixel(x, y, Rgb(color));
        }
    }
    for y in (height / 2)..height {
        let color = generate_random_color();
        for x in (width / 2)..width {
            image.put_pixel(x, y, Rgb(color));
        }
    }
    // vertical lines
    for x in 0..(width / 2) {
        let color = generate_random_color();
        for y in (height / 2)..height {
            image.put_pixel(x, y, Rgb(color));
        }
    }
    for x in (width / 2)..width {
        let color = generate_random_color();
        for y in 0..(height / 2) {
            image.put_pixel(x, y, Rgb(color));
        }
    }
    image.into_vec()
}

pub fn animation_frames<'a>(width: u32, height: u32, n_frames: i32) -> Vec<Frame<'static>> {
    let mut frames: Vec<Frame> = Vec::new();
    // TODO: otimizar e talvez paralelizar
    for _i in 0..n_frames {
        let image_vec = image_vec(width, height);
        // CREATE FRAME
        let frame = Frame::from_rgb(width as u16, height as u16, &image_vec);
        frames.push(frame);
    }
    frames
}
