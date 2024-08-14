use std::f32::consts::PI;

use glm::{cos, Vec3};

pub struct Camera {
    /// Camera position in the world space.
    pub eye: Vec3,

    /// Point the camera is looking at.
    pub center: Vec3,

    /// What's the up vector of the camera.
    pub up: Vec3,
}

impl Camera {
    pub fn change_basis(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let changed_based = vector.x * right + vector.y * up - vector.z * forward;

        changed_based.normalize()
    }

    /// Rotates the Camera by a given delta_yaw and pitch
    ///
    /// * `delta_yaw`: Rotates cam from left to right.
    /// * `delta_pitch`: Rotates cam up and down.
    pub fn rotate_cam(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();

        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Keep between [0, PI/2.0]
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        // Keep it between slightly below (-PI/2, PI/2)
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );

        self.eye = new_eye;
    }
}
