use images::{create_im::hori_vert_lines, GifAnimation};

#[test]
fn create_gif() {
    let mut im = hori_vert_lines::HoriVertLines::new(5, 250, 100);
    im.create_gif("hori_vert_test.gif");
}
