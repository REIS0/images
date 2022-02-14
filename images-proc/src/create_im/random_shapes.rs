use crate::{rgbimage::RgbImage, utils::generate_random_color};
use gif::Frame;
use rand::Rng;

fn random_shape(image: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    let (image_width, image_height) = image.dimensions();

    // DRAW shape
    // generate random image coordinates
    let shape_coord_x_begin = rng.gen_range(0..image_width);
    let shape_coord_x_end = rng.gen_range(shape_coord_x_begin..image_width);
    let shape_coord_y_begin = rng.gen_range(0..image_height);
    let shape_coord_y_end = rng.gen_range(shape_coord_y_begin..image_height);

    let shape_color = generate_random_color();

    // draw the new shape
    for x in shape_coord_x_begin..shape_coord_x_end {
        for y in shape_coord_y_begin..shape_coord_y_end {
            image.put_pixel(x, y, shape_color);
        }
    }
}

pub fn animation_frames(width: usize, height: usize, n_frames: usize) -> Vec<Frame<'static>> {
    let mut frames: Vec<Frame> = Vec::new();
    let mut image = RgbImage::new(width, height);

    // OPERATION
    for _i in 1..n_frames {
        random_shape(&mut image);
        // CREATE FRAME
        let image_vec = image.as_vec();
        let frame = Frame::from_rgb(width as u16, height as u16, &image_vec);
        frames.push(frame);
    }
    frames
}
