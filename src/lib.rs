use camera::Camera;
use cube::Cube;
use light::Light;
use sphere::Sphere;
use texture::GameTextures;

pub mod bmp;
pub mod camera;
pub mod color;
pub mod cube;
pub mod framebuffer;
pub mod light;
pub mod raytracer;
pub mod render;
pub mod sphere;
pub mod texture;

extern crate nalgebra_glm as glm;

pub fn are_equal(first: f32, second: f32, eps: f32) -> bool {
    (first - second).abs() <= eps
}

pub struct Model {
    pub spheres: Vec<Sphere>,
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub textures: GameTextures,
}

pub enum Message {
    RotateCamera(f32, f32),
    ZoomCamera(f32),
}
