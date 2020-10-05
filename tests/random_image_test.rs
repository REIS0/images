use images::{GifAnimation, create_im::random_shapes};

#[test]
fn create_gif() {
    let mut im = random_shapes::RandomShapesImage::new(5, 250, 100);
    im.create_gif("random_shape_test.gif");
}


