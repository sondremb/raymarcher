use crate::linalg::Vec3;

pub struct Scene(Vec<Box<Object>>);

impl Scene {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn distance_estimator(&self, z: &Vec3) -> f64 {
        self.0.iter().map(|obj| obj.distance_estimator(z)).fold(1./0., f64::min)
    }

    pub fn add<O: Object + 'static>(&mut self, obj: O) {
        self.0.push(Box::new(obj));
    }
}

pub trait Object {
    fn distance_estimator(&self, z: &Vec3) -> f64;
}

pub struct Union<T: Object, U: Object>(pub T, pub U);

impl<T: Object, U: Object> Object for Union<T, U> {
    fn distance_estimator(&self, z: &Vec3) -> f64 {
        self.0.distance_estimator(z).min(self.1.distance_estimator(z))
    }
}
pub struct Subtract<T: Object, U: Object>(pub T, pub U);

impl<T: Object, U: Object> Object for Subtract<T, U> {
    fn distance_estimator(&self, z: &Vec3) -> f64 {
        self.0.distance_estimator(z).max(-self.1.distance_estimator(z))
    }
}

pub struct Intersect<T: Object, U: Object>(pub T, pub U);

impl<T: Object, U: Object> Object for Intersect<T, U> {
    fn distance_estimator(&self, z: &Vec3) -> f64 {
        self.0.distance_estimator(z).max(self.1.distance_estimator(z))
    }
}