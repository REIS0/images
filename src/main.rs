extern crate image;
extern crate rand;

use ::image::{ImageBuffer, Rgb};
use ::rand::Rng;

macro_rules! rng {
    () => {
        rand::thread_rng().gen();
    };
}

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

fn main() {
    let mut image = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..(WIDTH / 2) {
        let color = generate_random_color();
        for x in 0..(HEIGHT / 2) {
            image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
        }
        let color = generate_random_color();
        for x in (HEIGHT / 2)..HEIGHT {
            image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
        }
    }

    for y in (WIDTH / 2)..WIDTH {
        let color = generate_random_color();
        for x in 0..(HEIGHT / 2) {
            image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
        }
        let color = generate_random_color();
        for x in (HEIGHT / 2)..HEIGHT {
            image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
        }
    }

    image.save("teste.png").unwrap();
}

// RGB
fn generate_random_color() -> (u8, u8, u8) {
    let red: u8 = rng!();
    let green: u8 = rng!();
    let blue: u8 = rng!();
    return (red, green, blue);
}
