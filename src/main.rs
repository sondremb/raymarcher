extern crate image;

#[macro_use]
pub mod linalg;

fn main() {
    use linalg::*;
    let vec = vec3!(1, 2, 3);
    assert_eq!(vec, Vec3::new(1., 2., 3.));
}