use glm::{Vec2, Vec3};

use crate::{
    raytracer::{Intersect, Material, Traceable},
    texture::CubeFace,
};

#[derive(Debug)]
pub struct Sphere {
    pub id: u32,
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Sphere {}

impl Traceable for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let oc = ray_origin - self.center;

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);

            if t > 0.0 {
                let distance = t;
                let point = ray_origin + ray_direction * t;
                let normal = (point - self.center).normalize();

                Some(Intersect {
                    distance,
                    material: self.material.clone(),
                    point,
                    normal,
                    face: CubeFace::NONE,
                    texture_cords: Vec2::zeros(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
