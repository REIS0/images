use crate::generate_random_color;
use crate::GifAnimation;
use gif::{Encoder, Frame, Repeat, SetParameter};
use image::{Rgb, RgbImage};
use rand::Rng;
use std::fs::File;

pub struct RandomShapesImage {
    n_frames: i32,
    width: u32,
    height: u32,
    rgb_image: RgbImage,
}

impl RandomShapesImage {
    pub fn new(n_frames: i32, width: u32, height: u32) -> RandomShapesImage {
        RandomShapesImage {
            n_frames,
            width,
            height,
            rgb_image: RgbImage::new(width, height),
        }
    }
    fn random_shape(&mut self) {
        let mut rng = rand::thread_rng();
        // DRAW shape
        // generate random image coordinates
        let shape_coord_x_begin = rng.gen_range(0, self.width);
        let shape_coord_x_end = rng.gen_range(shape_coord_x_begin, self.width);
        let shape_coord_y_begin = rng.gen_range(0, self.height);
        let shape_coord_y_end = rng.gen_range(shape_coord_y_begin, self.height);
        // generate shape color
        let shape_color = generate_random_color();
        // draw the new shape
        for x in shape_coord_x_begin..shape_coord_x_end {
            for y in shape_coord_y_begin..shape_coord_y_end {
                self.rgb_image.put_pixel(x, y, Rgb(shape_color));
            }
        }
    }
}

impl GifAnimation for RandomShapesImage {
    fn animation_frames<'a>(&mut self) -> Vec<Frame<'static>> {
        let mut frames: Vec<Frame> = Vec::new();
        // OPERATION
        for _i in 0..self.n_frames {
            self.random_shape();
            let frame = Frame::from_rgb(
                self.width as u16,
                self.height as u16,
                // TODO: change clone to more optimized function
                // ? multiple references?
                &self.rgb_image.clone().into_vec(),
            );
            frames.push(frame);
        }
        frames
    }

    fn create_gif(&mut self, nome: &str) {
        let frames = self.animation_frames();
        let mut animation = File::create(nome.to_owned() + ".gif").unwrap();
        let mut encoder = Encoder::new(
            &mut animation,
            self.width as u16,
            self.height as u16,
            &[0, 0, 0],
        )
        .unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        // TODO: map filter
        frames.iter().map(|f| encoder.write_frame(&f).unwrap());
    }
}
