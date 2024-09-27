use glm::Vec3;

use crate::light::{self, Light};
use crate::Model;
use crate::{color::Color, framebuffer::Framebuffer};

use crate::raytracer::Traceable;

pub fn init_render(framebuffer: &mut Framebuffer, data: &Model) {
    render(framebuffer, data);
}

pub fn cast_ray<T: Traceable + Send>(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[T],
    lights: &[Light],
) -> Color {
    let (intersect, _) = objects
        .iter()
        .flat_map(|object| object.ray_intersect(ray_origin, ray_direction))
        .fold((None, f32::INFINITY), |accum, intersection| {
            if intersection.distance < accum.1 {
                let distance = intersection.distance;
                (Some(intersection), distance)
            } else {
                accum
            }
        });

    if let Some(intersect) = intersect {
        lights.iter().fold(Color::default(), |previous, current| {
            let light_dir = (current.position - intersect.point).normalize();
            let diffuse_intensity = intersect.normal.dot(&light_dir);
            let diffuse = intersect.material.diffuse * diffuse_intensity * current.intensity;
            previous + diffuse
        })
    } else {
        Color::default().into()
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
            let pixel_color = cast_ray(
                &data.camera.eye,
                &rotated_direction,
                &data.spheres,
                &data.lights,
            );

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            let _ = framebuffer.paint_point(nalgebra_glm::Vec3::new(x as f32, y as f32, 0.0));
        }
    }
}
