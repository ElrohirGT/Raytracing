use glm::Vec3;

use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse: Color::default(),
        }
    }
}

pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub const EMPTY_INTERSECT: Intersect = Intersect {
    distance: 0.0,
    point: Vec3::new(0.0, 0.0, 0.0),
    normal: Vec3::new(0.0, 0.0, 0.0),
    material: Material {
        diffuse: Color::default(),
    },
};

pub trait Traceable {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
