use glm::{Vec2, Vec3};

use crate::{
    color::Color,
    material::Material,
    texture::{CubeFace, Textures},
};

#[derive(Debug)]
pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub texture_cords: Vec2,
    pub face: CubeFace,
}

pub const EMPTY_INTERSECT: Intersect = Intersect {
    distance: 0.0,
    point: Vec3::new(0.0, 0.0, 0.0),
    normal: Vec3::new(0.0, 0.0, 0.0),
    material: Material::default(),
    texture_cords: Vec2::new(0.0, 0.0),
    face: CubeFace::TOP,
};

pub trait Traceable {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
