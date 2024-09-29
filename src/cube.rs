use glm::{Vec2, Vec3};
use rand::Rng;

use crate::raytracer::{Intersect, Material, Traceable};

#[derive(Debug)]
pub struct Cube {
    pub id: u32,
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
    face_normals: Vec<Vec3>,
}

#[derive(Debug)]
pub struct BoxBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Cube {
    /// Creates a new Cube
    ///
    /// * `id`: The Unique identifier of the cube object.
    /// * `center`: The point at the center of the cube.
    /// * `size`: The length of one of the sizes of the cube.
    /// * `material`: The Material of which is the cube made of.
    /// * `up`: In what direction is up for the cube?
    pub fn new(id: u32, center: Vec3, size: f32, material: Material, up: Vec3) -> Self {
        // The y axis affects forwards and backwards movement.
        let original_forwards = Vec3::new(0.0, 1.0, 0.0);

        let right = original_forwards.cross(&up).normalize();
        let backwards = right.cross(&up).normalize();

        let face_normals = vec![
            up,         // up
            -up,        // down
            right,      // right
            -right,     // left
            -backwards, // forwards
            backwards,  // backwards
        ];

        Cube {
            id,
            center,
            size,
            material,
            face_normals,
        }
    }

    /// Get's an array that represents the bounds of the cube.
    /// The index 0 represents the x bounds.
    /// The index 1 represents the y bounds.
    /// The index 2 represents the z bounds.
    pub fn get_bounds(&self) -> BoxBounds {
        let half_size = self.size / 2.0;
        let center = self.center;

        let min = Vec3::new(
            center.x - half_size,
            center.y - half_size,
            center.z - half_size,
        );
        let max = Vec3::new(
            center.x + half_size,
            center.y + half_size,
            center.z + half_size,
        );

        BoxBounds { min, max }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Cube {}

impl Traceable for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        // Algorithm base on:
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection.html
        let cube_bounds = self.get_bounds();

        //         println!(
        //             r#"Checking if ray:
        // origin: {:?}
        // direction: {:?}
        // Collides!"#,
        //             ray_origin, ray_direction
        //         );

        let t0x = (cube_bounds.min.x - ray_origin.x) / ray_direction.x;
        let t1x = (cube_bounds.max.x - ray_origin.x) / ray_direction.x;

        let (txmin, txmax) = minmax(t0x, t1x);
        let (mut tmin, mut tmax) = (txmin, txmax);

        let t0y = (cube_bounds.min.y - ray_origin.y) / ray_direction.y;
        let t1y = (cube_bounds.max.y - ray_origin.y) / ray_direction.y;

        let (tymin, tymax) = minmax(t0y, t1y);

        if tmin > tymax || tymin > tmax {
            return None;
        }

        tmin = tmin.max(tymin);
        tmax = tmin.min(tymax);

        let t0z = (cube_bounds.min.z - ray_origin.z) / ray_direction.z;
        let t1z = (cube_bounds.max.z - ray_origin.z) / ray_direction.z;

        let (tzmin, tzmax) = minmax(t0z, t1z);

        if tmin > tzmax || tzmin > tmax {
            //             println!(
            //                 r#"No collision found!
            // {} > {} || {} > {}
            // "#,
            //                 tmin, tzmax, tzmin, tmax
            //             );
            return None;
        }

        // Both this values are the same...
        tmin = tmin.max(tzmin);
        // tmax = tmax.min(tzmax);

        let point = ray_origin + ray_direction * tmin;

        let normal: Vec3 = self
            .face_normals
            .iter()
            .filter(|face_normal| face_normal.dot(ray_direction) < 0.0)
            .take(1)
            .cloned()
            .next()
            .unwrap();

        let distance = glm::distance(ray_origin, &point);
        let intersect = Intersect {
            distance,
            point,
            normal,
            material: self.material,
        };

        //         println!();
        //         println!(
        //             r#"Found collisions:
        // x: ({}, {})
        // y: ({}, {})
        // z: ({}, {})
        // Which generate the Point: {:?}"#,
        //             txmin, txmax, tymin, tymax, tzmin, tzmax, point
        //         );
        //         println!("Found an intersection! {:#?}", intersect);

        Some(intersect)
    }
}

/// Computes the minimum and maximum of two values.
/// Returns: A tuple in the form of (min, max).
fn minmax(a: f32, b: f32) -> (f32, f32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
