extern crate image;

#[macro_use]
pub mod linalg;
pub mod render;

fn main() {
    use linalg::*;
    use render::*;

    let image_path = "out.png";
    let port = ViewPort::new(
        &vec3!(0, 0, 10),
        &vec3!(0, 0, -1),
        &vec3!(0, 1, 0),
        1280,
        720,
        100.0
    );
    let image = render(&port);
    image.save(image_path).unwrap();
}