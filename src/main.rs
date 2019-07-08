extern crate image;

#[macro_use]
pub mod linalg;
pub mod render;
pub mod objects;

fn main() {
    use linalg::*;
    use render::*;
    use objects::*;

    let image_path = "out.png";
    let port = ViewPort::new(
        &vec3!(3, 3, 3),
        &vec3!(-2, -3, -3),
        &vec3!(0, 1, 0),
        1280,
        720,
        100.0
    );
    let mut scene = Scene::new();
    let b = Cube::new(&vec3!(0.75));
    let s = Sphere::new(1.0);
    scene.add(Subtract(b, s));
    let image = render(&port, &scene);
    image.save(image_path).unwrap();
}