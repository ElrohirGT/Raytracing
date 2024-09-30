use crate::{color::Color, texture::Textures};

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub texture: Option<Textures>,
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
            texture: None,
        }
    }
}

pub const DIRT: Material = Material {
    diffuse: Color::pink(),
    specular: 1.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.42,
    texture: Some(Textures::DIRT),
};

pub const STONE: Material = Material {
    diffuse: Color::pink(),
    specular: 1.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.42,
    texture: Some(Textures::STONE),
};
