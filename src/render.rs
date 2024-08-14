use glm::Vec3;

use crate::Model;
use crate::{color::Color, framebuffer::Framebuffer};

use crate::raytracer::Traceable;

pub fn init_render(framebuffer: &mut Framebuffer, data: &Model) {
    render(framebuffer, data);
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[impl Traceable]) -> Color {
    let mut intersect = None;
    let mut zbuffer = f32::NEG_INFINITY;

    for object in objects {
        let potential_intersect = object.ray_intersect(ray_origin, ray_direction);
        if let Some(actual_intersect) = potential_intersect {
            if actual_intersect.distance > zbuffer {
                zbuffer = actual_intersect.distance;
                intersect = Some(actual_intersect);
            }
        }
    }

    if let Some(intersect) = intersect {
        intersect.material.diffuse
    } else {
        0x000000.into()
    }
}

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    framebuffer.clear();

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

            // Cast the ray and get the pixel color
            let rotated_direction = data.camera.change_basis(&ray_direction);
            let pixel_color = cast_ray(&data.camera.eye, &rotated_direction, &data.spheres);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            let _ = framebuffer.paint_point(nalgebra_glm::Vec3::new(x as f32, y as f32, 0.0));
        }
    }
}
