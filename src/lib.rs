use camera::Camera;
use cube::Cube;
use glm::Vec3;
use light::{AmbientLightIntensity, Light};
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

/// Computes the minimum and maximum of two values.
/// Returns: A tuple in the form of (min, max).
pub fn minmax(a: f32, b: f32) -> (f32, f32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub struct Model {
    pub spheres: Vec<Sphere>,
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
    pub ambient_light: AmbientLightIntensity,
    pub camera: Camera,
    pub textures: GameTextures,
}

pub enum Message {
    RotateCamera(f32, f32),
    ZoomCamera(f32),
    MoveFocus(Vec3),
}
