use gif::Frame;
use rand::Rng;

pub mod create_im;

macro_rules! rng {
    () => {
        rand::thread_rng().gen();
    };
}

// return a RGB array
pub fn generate_random_color() -> [u8; 3] {
    let red: u8 = rng!();
    let green: u8 = rng!();
    let blue: u8 = rng!();
    [red, green, blue]
}

pub trait GifAnimation {
    fn create_gif(&mut self, nome: &str);

    fn animation_frames<'a>(&mut self) -> Vec<Frame<'static>>;
}
