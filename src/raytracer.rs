use glm::Vec3;

pub trait Traceable {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Traceable for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool {
        let oc = ray_origin - self.center;

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}
