use image::{DynamicImage, GenericImage, Rgba};
use crate::linalg::Vec3;
use crate::objects::Scene;

pub struct ViewPort {
    pub pos: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    pub width: u32,
    pub height: u32,
    delta: f64
}

impl ViewPort {
    pub fn new(pos: &Vec3, forward: &Vec3, up: &Vec3, width: u32, height: u32, fov: f64) -> Self {
        let forward = forward.normalized();
        let right = forward.cross(&up).normalized();
        let up = right.cross(&forward).normalized();
        let delta = 2.0 * (fov.to_radians() / 2.0).tan() / (width as f64);
        ViewPort {
            pos: pos.clone(),
            forward,
            up,
            right,
            width,
            height,
            delta
        }
    }

    pub fn ray_from_pixel(&self, x: u32, y: u32) -> Vec3 {
        (self.forward 
            + self.right * self.delta * (x as f64 - self.width as f64 / 2.0)
            - self.up * self.delta * (y as f64 - self.height as f64 / 2.0))
            .normalized()
    }
}

pub fn render(port: &ViewPort, scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_luma8(port.width, port.height);
    for y in 0..port.height {
        for x in 0..port.width {
            let ray = port.ray_from_pixel(x, y);
            image.put_pixel(x, y, cast_ray(&port.pos, &ray, scene));
        }
    }
    image
}

fn cast_ray(from: &Vec3, dir: &Vec3, scene: &Scene) -> Rgba<u8> {
    const max_steps: u8 = 100;
    let mut total_dist = 0.0;
    let mut steps = 0;
    for i in 0..max_steps {
        steps = i;
        let p = *from + *dir * total_dist;
        let dist = scene.distance_estimator(&p);
        total_dist += dist;
        if dist < 0.0001 {
            break;
        }
    }
    let intensity = 1.0 - steps as f64 / max_steps as f64;
    let intensity = (intensity * 255.0) as u8;
    Rgba([intensity, intensity, intensity, 255])
}