use images_proc::{create_im, errors::GifGenerationError, utils::create_gif};
use std::io::stdin;

pub struct Args {
    pub n_frames: usize,
    pub height: usize,
    pub width: usize,
    pub proc: String,
    pub name: String,
}

fn main() -> Result<(), GifGenerationError> {
    let args = get_input();

    println!("Creating frames...");
    let frames;
    if args.proc == "lines" {
        frames =
            create_im::hori_vert_lines::animation_frames(args.width, args.height, args.n_frames);
    } else {
        frames = create_im::random_shapes::animation_frames(args.width, args.height, args.n_frames);
    }

    println!("Creating gif...");
    create_gif(args.width as u16, args.height as u16, frames, args.name)?;

    Ok(())
}

fn get_input() -> Args {
    // Input the number of frames
    println!("Number of frames: ");
    let mut n_frames = String::new();
    stdin()
        .read_line(&mut n_frames)
        .expect("Failed to read line.");
    let n_frames = n_frames
        .trim()
        .parse::<usize>()
        .expect("Insert a valid integer value.");

    println!("Height: ");
    let mut height = String::new();
    stdin()
        .read_line(&mut height)
        .expect("Failed to read line.");
    let height = height
        .trim()
        .parse::<usize>()
        .expect("Insert valid integer value.");

    println!("Width: ");
    let mut width = String::new();
    stdin().read_line(&mut width).expect("Failed to read line.");
    let width = width
        .trim()
        .parse::<usize>()
        .expect("Insert valid integer value");

    println!("Type ('lines', 'shapes': ");
    let mut proc = String::new();
    stdin().read_line(&mut proc).expect("Failed to read line.");
    let proc = proc.trim().to_string();

    println!("File name: ");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line.");
    let name = name.trim().to_string();

    Args {
        n_frames,
        height,
        width,
        proc,
        name,
    }
}
