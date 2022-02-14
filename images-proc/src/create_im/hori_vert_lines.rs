use gif::Frame;
// local import
use crate::{rgbimage::RgbImage, utils::generate_random_color};

fn image_vec(width: usize, height: usize) -> Vec<u8> {
    let mut image = RgbImage::new(width, height);

    // horizontal lines
    for y in 0..(height / 2) {
        let color = generate_random_color();
        for x in 0..(width / 2) {
            image.put_pixel(x, y, color);
        }
    }
    for y in (height / 2)..height {
        let color = generate_random_color();
        for x in (width / 2)..width {
            image.put_pixel(x, y, color);
        }
    }

    // vertical lines
    for x in 0..(width / 2) {
        let color = generate_random_color();
        for y in (height / 2)..height {
            image.put_pixel(x, y, color);
        }
    }
    for x in (width / 2)..width {
        let color = generate_random_color();
        for y in 0..(height / 2) {
            image.put_pixel(x, y, color);
        }
    }

    image.as_vec()
}

pub fn animation_frames<'a>(width: usize, height: usize, n_frames: usize) -> Vec<Frame<'static>> {
    let mut frames: Vec<Frame> = Vec::new();
    for _i in 0..n_frames {
        let image_vec = image_vec(width, height);
        // CREATE FRAME
        let frame = Frame::from_rgb(width as u16, height as u16, &image_vec);
        frames.push(frame);
    }
    frames
}
