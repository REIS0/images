use gif::{Encoder, Frame, Repeat, SetParameter};
use rand::Rng;
use std::fs::File;

macro_rules! rng {
    () => {
        rand::thread_rng().gen();
    };
}

// RGB
pub fn generate_random_color() -> (u8, u8, u8) {
    let red: u8 = rng!();
    let green: u8 = rng!();
    let blue: u8 = rng!();
    return (red, green, blue);
}

// create the gif animation
pub fn create_gif(width: u16, height: u16, frames: Vec<Frame>, nome: &str) {
    // animation
    let mut animation = File::create(nome.to_owned() + ".gif").unwrap();
    let mut encoder = Encoder::new(&mut animation, width, height, &[0, 0, 0]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for f in frames {
        encoder.write_frame(&f).unwrap();
    }
}

