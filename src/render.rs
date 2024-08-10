use std::time::{SystemTime, UNIX_EPOCH};

use glm::Vec3;
use nalgebra_glm::vec2_to_vec3;

use crate::{
    color::Color,
    framebuffer::Framebuffer,
    raycaster::{cast_ray_2d, cast_ray_3d},
    texture::{GameTextures, Texture},
    BoardCell, GameStatus, Model,
};

use crate::raytracer::Traceable;

fn from_char_to_texture<'a>(c: &BoardCell, textures: &'a GameTextures) -> Option<&'a Texture> {
    match c {
        BoardCell::HorizontalWall => Some(&textures.horizontal_wall),
        BoardCell::VerticalWall => Some(&textures.vertical_wall),
        BoardCell::PillarWall => Some(&textures.corner_wall),
        _ => None,
    }
}

fn from_cell_to_color(c: &BoardCell) -> Color {
    match c {
        BoardCell::HorizontalWall | BoardCell::VerticalWall | BoardCell::PillarWall => 0xff00ff,
        _ => 0xffffff,
    }
    .into()
}

pub fn init_render(framebuffer: &mut Framebuffer, data: &Model) {
    render(framebuffer, data);
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: [&impl Traceable]) -> Color {
    for object in objects {
        if object.ray_intersect(ray_origin, ray_direction) {
            return 0xffffff;
        }
    }

    0x000000
}

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    framebuffer.clear();

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..height {
        for x in 0..width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}
