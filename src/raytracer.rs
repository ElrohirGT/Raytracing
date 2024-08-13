use glm::Vec3;

use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

pub struct Intersect {
    pub distance: f32,
    pub material: Material,
}

pub trait Traceable {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
