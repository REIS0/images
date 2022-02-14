#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct RgbImage {
    image: Vec<Vec<Rgb>>,
    width: usize,
    height: usize,
}

impl RgbImage {
    pub fn new(width: usize, height: usize) -> Self {
        RgbImage {
            image: vec![
                vec![
                    Rgb {
                        red: 0,
                        green: 0,
                        blue: 0
                    };
                    width
                ];
                height
            ],
            width,
            height,
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: Rgb) {
        self.image[y][x] = pixel;
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut image_vec = Vec::new();
        for line in &self.image {
            for pixel in line {
                image_vec.push(pixel.red);
                image_vec.push(pixel.green);
                image_vec.push(pixel.blue);
            }
        }
        image_vec
    }
}
