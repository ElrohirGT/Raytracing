use std::f32::consts::PI;

use glm::Vec3;

pub struct Camera {
    /// Determines whether or not the camera has changed.
    has_changed: bool,

    /// Camera position in the world space.
    pub eye: Vec3,

    /// Point the camera is looking at.
    pub center: Vec3,

    /// What's the up vector of the camera.
    pub up: Vec3,
}

impl Camera {
    /// Creates a new Camera with the given parameters.
    ///
    /// * `eye`: Camera position in the world space.
    /// * `center`: Point the camera is looking at.
    /// * `up`: What's the up vector of the camera.
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }
    pub fn change_basis(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let changed_based = vector.x * right + vector.y * up - vector.z * forward;

        changed_based.normalize()
    }

    /// Advances the Camera by a given delta.
    pub fn zoom_cam(&mut self, delta: f32) {
        self.has_changed = true;
        let forward_dir = (self.center - self.eye).normalize();
        self.eye += forward_dir * delta;
    }

    /// Zooms in and zooms out the camera by a given delta.
    pub fn zoom(&mut self, delta: f32) {
        self.has_changed = true;
        let forward_dir = (self.center - self.eye).normalize();
        self.eye += forward_dir * delta;
    }

    /// Rotates the Camera by a given delta_yaw and pitch
    ///
    /// * `delta_yaw`: Rotates cam from left to right.
    /// * `delta_pitch`: Rotates cam up and down.
    pub fn rotate_cam(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.has_changed = true;
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

    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub fn reset_change(&mut self) {
        self.has_changed = false;
    }

    pub fn move_focus(&mut self, delta_pos: Vec3) {
        self.has_changed = true;
        self.center += delta_pos;
    }

    pub fn direction(&self) -> Vec3 {
        (self.center - self.eye).normalize()
    }
}
