use glm::Vec3;

use crate::light::{self, Light};
use crate::Model;
use crate::{color::Color, framebuffer::Framebuffer};

use crate::raytracer::{Intersect, Traceable};

pub fn init_render(framebuffer: &mut Framebuffer, data: &Model) {
    render(framebuffer, data);
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow<'a, T: Traceable + 'a, ObIterable: Iterator<Item = &'a T>>(
    intersect: &Intersect,
    light: &Light,
    objects: ObIterable,
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point;
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if let Some(object_position) = shadow_intersect {
            let distance_to_light = nalgebra_glm::distance2(&light.position, &intersect.point);
            let distance_from_object_to_light =
                nalgebra_glm::distance2(&light.position, &object_position.point);
            shadow_intensity = distance_from_object_to_light / distance_to_light;
            break;
        }
    }

    shadow_intensity
}

pub fn cast_ray<T: Traceable + Eq>(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[T],
    lights: &[Light],
) -> Color {
    let (intersect, _, impact_object) = objects
        .iter()
        .flat_map(|object| {
            object
                .ray_intersect(ray_origin, ray_direction)
                .map(|inter| (inter, object))
        })
        .fold((None, f32::INFINITY, None), |accum, intersection| {
            if intersection.0.distance < accum.1 {
                let distance = intersection.0.distance;
                (Some(intersection.0), distance, Some(intersection.1))
            } else {
                accum
            }
        });

    if let (Some(intersect), Some(impact_object)) = (intersect, impact_object) {
        lights
            .iter()
            .fold(Color::default(), |accumulator_color, current_light| {
                let light_dir = (current_light.position - intersect.point).normalize();
                let view_dir = (ray_origin - intersect.point).normalize();
                let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
                let shadow_intensity = cast_shadow(
                    &intersect,
                    current_light,
                    objects.iter().filter(|o| o != &impact_object),
                );
                let light_intensity = current_light.intensity * (1.0 - shadow_intensity);

                let diffuse_intensity = intersect.normal.dot(&light_dir).clamp(0.0, 1.0);
                let diffuse = intersect.material.diffuse
                    * intersect.material.albedo
                    * diffuse_intensity
                    * light_intensity;

                let specular_intensity = view_dir
                    .dot(&reflect_dir)
                    .clamp(0.0, 1.0)
                    .powf(intersect.material.specular);
                let specular = current_light.color
                    * intersect.material.reflectivity
                    * specular_intensity
                    * light_intensity;

                accumulator_color + diffuse + specular
            })
    } else {
        // Sky color...
        0x181818.into()
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
