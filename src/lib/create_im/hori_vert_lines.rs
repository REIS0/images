use std::fs::File;

use gif::{Encoder, Frame, Repeat, SetParameter};
use image::{Rgb, RgbImage};
// local import
use crate::{generate_random_color, GifAnimation};

// ? talvez usar u16 inves de u32
pub struct HoriVertLines {
    n_frames: i32,
    width: u32,
    height: u32,
    rgb_image: RgbImage,
}

impl HoriVertLines {
    pub fn new(n_frames: i32, width: u32, height: u32) -> HoriVertLines {
        HoriVertLines {
            n_frames,
            width,
            height,
            rgb_image: RgbImage::new(width, height),
        }
    }

    fn image_vec(&mut self) {
        // horizontal lines
        for y in 0..(self.height / 2) {
            let color = generate_random_color();
            // TODO: parallel loop
            for x in 0..(self.width / 2) {
                self.rgb_image.put_pixel(x, y, Rgb(color));
            }
        }
        for y in (self.height / 2)..self.height {
            let color = generate_random_color();
            // TODO: parallel loop
            for x in (self.width / 2)..self.width {
                self.rgb_image.put_pixel(x, y, Rgb(color));
            }
        }
        // vertical lines
        for x in 0..(self.width / 2) {
            let color = generate_random_color();
            // TODO: parallel loop
            for y in (self.height / 2)..self.height {
                self.rgb_image.put_pixel(x, y, Rgb(color));
            }
        }
        for x in (self.width / 2)..self.width {
            let color = generate_random_color();
            // TODO: parallel loop
            for y in 0..(self.height / 2) {
                self.rgb_image.put_pixel(x, y, Rgb(color));
            }
        }
    }
}

impl GifAnimation for HoriVertLines {
    fn create_gif(&mut self, nome: &str) {
        let frames = self.animation_frames();
        let mut animation = File::create(nome).unwrap();
        let mut encoder = Encoder::new(
            &mut animation,
            self.width as u16,
            self.height as u16,
            &[0, 0, 0],
        )
        .unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        for frame in frames {
            encoder.write_frame(&frame).ok();
        }
    }

    fn animation_frames<'a>(&mut self) -> Vec<Frame<'static>> {
        let mut frames: Vec<Frame> = Vec::new();

        for _i in 0..self.n_frames {
            self.image_vec();
            let frame = Frame::from_rgb(
                self.width as u16,
                self.height as u16,
                &self.rgb_image.clone().into_vec(),
            );
            frames.push(frame);
        }
        frames
    }
}
