use crate::{errors::GifGenerationError, rgbimage::Rgb};
use gif::{Encoder, Frame, Repeat};
use rand::{thread_rng, Rng};
use std::fs::File;

pub fn generate_random_color() -> Rgb {
    let mut rng = thread_rng();

    Rgb {
        red: rng.gen(),
        green: rng.gen(),
        blue: rng.gen(),
    }
}

pub fn create_gif(
    width: u16,
    height: u16,
    frames: Vec<Frame>,
    name: String,
) -> Result<(), GifGenerationError> {
    match File::create(name + ".gif") {
        Ok(mut file) => match Encoder::new(&mut file, width, height, &[0, 0, 0]) {
            Ok(mut enconder) => {
                if let Err(e) = enconder.set_repeat(Repeat::Infinite) {
                    return Err(GifGenerationError::EncondingError(e));
                };
                for f in frames {
                    match enconder.write_frame(&f) {
                        Ok(()) => continue,
                        Err(e) => GifGenerationError::EncondingError(e),
                    };
                }
                Ok(())
            }
            Err(e) => Err(GifGenerationError::EncondingError(e)),
        },
        Err(e) => Err(GifGenerationError::FileCreationError(e)),
    }
}
