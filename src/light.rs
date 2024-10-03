use crate::color::Color;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}

pub type AmbientLightIntensity = f32;
