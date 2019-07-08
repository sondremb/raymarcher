use crate::linalg::Vec3;
use crate::objects::base::Object;

pub struct Sphere {
    r: f64
}

impl Sphere {
    pub fn new(r: f64) -> Self {
        Self {r}
    }
}

impl Object for Sphere {
    fn distance_estimator(&self, z: &Vec3) -> f64 {
        z.len() - self.r
    }
}

// techincally not a cube since it's not necessarliy equilateral
// but the name Box was taken by Rust, so Cube it is
pub struct Cube {
    b: Vec3
}

impl Cube {
    pub fn new(b: &Vec3) -> Self {
        Self {b: b.clone()}
    }
}

impl Object for Cube {
    fn distance_estimator(&self, z: &Vec3) -> f64 {
        let d = z.abs() - self.b;
        d.max(0.).len() + d.y.max(d.z).max(d.x).min(0.)
    }
}