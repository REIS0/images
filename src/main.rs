extern crate gif;
extern crate image;
extern crate rand;

use ::image::{ImageBuffer, Rgb};
use ::rand::Rng;
use ::std::fs::File;
use::gif::{Frame, Encoder, Repeat, SetParameter};
use::std::io;

macro_rules! rng {
    () => {
        rand::thread_rng().gen();
    };
}

const WIDTH: u32 = 500; // x
const HEIGHT: u32 = 500; // y

fn main() {
    // Input the number of frames
    println!("Number of frames: ");
    let mut n_frames = String::new();
    io::stdin().read_line(&mut n_frames).expect("Failed to read line.");
    let n_frames: i32 = n_frames.trim().parse().expect("Insert a valid integer value."); 

    let mut frames: Vec<Frame> = Vec::new();

    // TODO: otimizar e talvez paralelizar
    println!("Creating frames...");
    for _i in 0..n_frames {
        // CREATE IMAGE
        let mut image = ImageBuffer::new(WIDTH, HEIGHT);
        // horizontal lines
        for y in 0..(HEIGHT / 2) {
            let color = generate_random_color();
            for x in 0..(WIDTH / 2) {
                image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
            }
        }
        for y in (HEIGHT / 2)..HEIGHT {
            let color = generate_random_color();
            for x in (WIDTH / 2)..WIDTH {
                image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
            }
        }
        // vertical lines
        for x in 0..(WIDTH / 2) {
            let color = generate_random_color();
            for y in (HEIGHT / 2)..HEIGHT {
                image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
            }
        }
        for x in (WIDTH / 2)..WIDTH {
            let color = generate_random_color();
            for y in 0..(HEIGHT / 2) {
                image.put_pixel(x, y, Rgb([color.0, color.1, color.2]))
            }
        }

        // CREATE FRAME
        let frame = Frame::from_rgb(WIDTH as u16, HEIGHT as u16, &image.into_vec());
        frames.push(frame);
    }

    println!("Creating gif...");
    create_gif(frames, "teste");
}

// RGB
fn generate_random_color() -> (u8, u8, u8) {
    let red: u8 = rng!();
    let green: u8 = rng!();
    let blue: u8 = rng!();
    return (red, green, blue);
}

// create the gif animation
fn create_gif(frames: Vec<Frame>, nome: &str) {
    // animation
    let mut animation = File::create(nome.to_owned() + ".gif").unwrap();
    let mut encoder = Encoder::new(&mut animation, WIDTH as u16, HEIGHT as  u16, &[0, 0, 0]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for f in frames {
        encoder.write_frame(&f).unwrap();  
    }
}

// TODO: efeito gradiente
// fn gradient_effect(image: ImageBuffer<>) -> ImageBuffer<> {
//
//
// return
// }
