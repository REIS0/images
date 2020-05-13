extern crate gif;
extern crate image;
extern crate rand;

use::std::io;
// local functions
use::images::bsc::create_gif;
use::images::create_im::hori_vert_lines::animation_frames;

const WIDTH: u32 = 500; // x
const HEIGHT: u32 = 500; // y

fn main() {
    // Input the number of frames
    println!("Number of frames: ");
    let mut n_frames = String::new();
    io::stdin().read_line(&mut n_frames).expect("Failed to read line.");
    let n_frames: i32 = n_frames.trim().parse().expect("Insert a valid integer value."); 


    println!("Creating frames...");
    let frames = animation_frames(WIDTH, HEIGHT, n_frames);

    println!("Creating gif...");
    create_gif(WIDTH as u16, HEIGHT as u16, frames,"teste");
}


// TODO: efeito gradiente
// fn gradient_effect(image: ImageBuffer<>) -> ImageBuffer<> {
//
//
// return
// }
