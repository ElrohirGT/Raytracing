use std::fmt::Debug;

use glm::Vec3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::light::{AmbientLightIntensity, Light};
use crate::texture::GameTextures;
use crate::{color::Color, framebuffer::Framebuffer};
use crate::{minmax, Model};

use crate::raytracer::{Intersect, Traceable};

pub fn init_render(framebuffer: &mut Framebuffer, data: &Model) {
    render(framebuffer, data);
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).clamp(-1.0, 1.0);
    // We assume the ray is leaving the object...
    let mut n_cosi = cosi;
    let mut eta = eta_t;
    let mut n_normal = *normal;

    // Ray is entering the object
    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
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
        if let Some(object_intersection) = shadow_intersect {
            if object_intersection.distance < 0.0 {
                break;
            }

            let distance_from_object_to_light =
                nalgebra_glm::distance2(&light.position, &object_intersection.point);
            shadow_intensity = object_intersection.distance / distance_from_object_to_light;

            break;
        }
    }

    shadow_intensity
}

const TARGET_COLOR: u32 = 0x030201;
pub fn cast_ray<T: Traceable + Eq + Debug>(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[T],
    lights: &[Light],
    ambient_light: AmbientLightIntensity,
    textures: &GameTextures,
    depth: u32,
) -> Color {
    let skybox_color = 0x383838.into();
    if depth > 3 {
        return skybox_color;
    }

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
                // if light_intensity < 0.0 {
                //     println!(
                //         "The light is negative! {} * (1.0 - {})",
                //         current_light.intensity, shadow_intensity
                //     )
                // }

                let diffuse_intensity =
                    intersect.normal.dot(&light_dir).clamp(0.0, 1.0) + ambient_light;
                let tx_color = match intersect.material.texture {
                    Some(tx_type) => {
                        let texture = textures.get_texture(&tx_type);
                        texture.get_color_of_face(
                            &intersect.face,
                            intersect.texture_cords.x * texture.sprite_size as f32,
                            intersect.texture_cords.y * texture.sprite_size as f32,
                        )
                    }
                    None => intersect.material.diffuse,
                };
                let diffuse =
                    tx_color * intersect.material.albedo.0 * diffuse_intensity * light_intensity;
                if diffuse == TARGET_COLOR.into() {
                    println!(
                        "Diffuse is black!\n{tx_color:?} * {} * {diffuse_intensity} * {light_intensity}",
                         intersect.material.albedo.0, 
                    )
                }

                let specular_intensity = view_dir
                    .dot(&reflect_dir)
                    .clamp(0.0, 1.0)
                    .powf(intersect.material.specular);
                let specular = current_light.color
                    * intersect.material.albedo.1
                    * specular_intensity
                    * light_intensity;

                let mut reflect_color = Color::black();
                let reflectivity = intersect.material.reflectivity;
                if reflectivity > 0.0 {
                    let reflect_dir = reflect(&-ray_direction, &intersect.normal).normalize();
                    // Tenemos que hacer offset para evitar el acné
                    let reflect_origin = intersect.point + 1e-2 * intersect.normal;
                    reflect_color = cast_ray(
                        &reflect_origin,
                        &reflect_dir,
                        objects,
                        lights,
                        ambient_light,
                        textures,
                        depth + 1,
                    )
                }

                let mut refract_color = Color::black();
                let transparency = intersect.material.transparency;
                if transparency < 0.0 {
                    let refract_dir = refract(
                        ray_direction,
                        &intersect.normal,
                        intersect.material.refractive_index,
                    );
                    // Tenemos que hacer offset para evitar el acné
                    let refract_origin = intersect.point + 1e-2 * intersect.normal;

                    refract_color = cast_ray(
                        &refract_origin,
                        &refract_dir,
                        objects,
                        lights,
                        ambient_light,
                        textures,
                        depth + 1,
                    );
                }

                let color = accumulator_color
                    + (diffuse + specular) * (1.0 - reflectivity - transparency)
                    + (reflect_color * reflectivity)
                    + (refract_color * transparency);

                if color == TARGET_COLOR.into() {
                    println!(
                        r#"Found target color! {color:?}
DIFFUSE:
intensity: {diffuse_intensity}
light_intensity: {light_intensity}

REFLECT:
reflectivity: {reflectivity}

REFRACT:
transparency: {transparency}

accum: {accumulator_color:?}
+ ({diffuse:?} + {specular:?}) * (1.0 - {reflectivity} - {transparency})
+ ({reflect_color:?} * {reflectivity})
+ ({refract_color:?} * {transparency})

Intersect: {intersect:#?}
ImpactObject: {impact_object:#?}
CurrentLight: {current_light:#?}
"#
                    );
                }

                color
            })
    } else {
        // Sky color...
        skybox_color
    }
}

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    framebuffer.clear();

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let pixel_colors: Vec<Color> = (0..framebuffer.height)
        .into_par_iter()
        .flat_map(|y| {
            (0..framebuffer.width).into_par_iter().map(move |x| {
                // Map the pixel coordinate to screen space [-1, 1]
                let screen_x = (2.0 * x as f32) / width - 1.0;
                let screen_y = -(2.0 * y as f32) / height + 1.0;

                // Adjust for aspect ratio
                let screen_x = screen_x * aspect_ratio;

                // Calculate the direction of the ray for this pixel
                let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

                // Cast the ray and get the pixel color
                let rotated_direction = data.camera.change_basis(&ray_direction);
                cast_ray(
                    &data.camera.eye,
                    &rotated_direction,
                    &data.cubes,
                    &data.lights,
                    data.ambient_light,
                    &data.textures,
                    0,
                )
            })
        })
        .collect();

    for (i, color) in pixel_colors.into_iter().enumerate() {
        framebuffer.set_current_color(color);
        let y = (i / framebuffer.width) as f32;
        let x = if i > framebuffer.width {
            i % framebuffer.width
        } else {
            i
        } as f32;
        let _ = framebuffer.paint_point(nalgebra_glm::Vec2::new(x, y));
    }
}
