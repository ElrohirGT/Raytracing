use camera::Camera;
use light::Light;
use sphere::Sphere;

pub mod bmp;
pub mod camera;
pub mod color;
pub mod framebuffer;
pub mod light;
pub mod raytracer;
pub mod render;
pub mod sphere;

extern crate nalgebra_glm as glm;

pub fn are_equal(first: f32, second: f32, eps: f32) -> bool {
    (first - second).abs() <= eps
}

pub struct Model {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

pub enum Message {
    RotateCamera(f32, f32),
    MoveCamera(f32),
}
