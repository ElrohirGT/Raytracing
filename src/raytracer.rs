use glm::Vec3;

use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    /// La cantidad de luz que un material absorbe, en porcentaje (0,1).
    /// y también
    /// La cantidad de luz que un material refleja de la fuente, en porcentaje (0,1).
    pub albedo: (f32, f32),
    /// La cantidad de luz que depende del entorno en porcentaje (0,1).
    pub reflectivity: f32,
    /// La cantidad de luz que depende de lo que se encuentra detrás del objeto en porcentaje (0,1).
    pub transparency: f32,
    /// El índice refractivo del material.
    pub refractive_index: f32,
}

impl Material {
    pub const fn default() -> Self {
        Material {
            diffuse: Color::default(),
            specular: 0.0,
            albedo: (0.0, 0.0),
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 0.0,
        }
    }
}

#[derive(Debug)]
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
    material: Material::default(),
};

pub trait Traceable {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
