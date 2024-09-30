use std::{fs::File, io::BufReader};

use glm::Vec2;
use image::{
    codecs::gif::GifDecoder, AnimationDecoder, Frame, GenericImageView, ImageDecoder, ImageReader,
    Pixel,
};

use crate::color::Color;

#[derive(Debug)]
pub enum CubeFace {
    NONE,
    TOP,
    BOTTOM,
    FORWARDS,
    BACKWARDS,
    LEFT,
    RIGHT,
}

pub struct GameTextures {
    pub dirt: Texture,
}

#[derive(Debug, Clone, Copy)]
pub enum Textures {
    DIRT,
}

impl GameTextures {
    pub fn new(asset_dir: &str) -> Self {
        let dirt = format!("{}{}", asset_dir, "dirt.png");

        let dirt = Texture::new(&dirt, 16);
        // let vertical_wall = Texture::new(&vertical_wall);
        // let corner_wall = Texture::new(&corner_wall);
        // let lolibunny = Texture::new(&lolibunny);
        // let start_screen = Texture::new(&start_screen);
        // let loose_screen = AnimatedTexture::new(&loose_screen);
        // let win_screen = AnimatedTexture::new(&win_screen);
        // let splash_screen = AnimatedTexture::new(&splash_screen);

        GameTextures { dirt }
    }

    pub fn get_texture(&self, tx_type: &Textures) -> &Texture {
        match tx_type {
            Textures::DIRT => &self.dirt,
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub sprite_size: usize,
    colors: Vec<Color>,
}

pub struct AnimatedTexture {
    pub width: u32,
    pub height: u32,
    frames: Vec<Frame>,
    pub frame_count: usize,
}

impl AnimatedTexture {
    pub fn new(file_path: &str) -> Self {
        let file_in = BufReader::new(File::open(file_path).unwrap());
        let decoder = GifDecoder::new(file_in).unwrap();
        let (width, height) = decoder.dimensions();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding gif");
        let frame_count = frames.len();

        Self {
            width,
            height,
            frames,
            frame_count,
        }
    }

    /// Get's the color of the pixel positioned on the frame `t`.
    pub fn get_pixel_color(&self, t: usize, x: u32, y: u32) -> Color {
        let pixel = self.frames[t].buffer().get_pixel(x, y).to_rgb();
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        Color { r, g, b }
    }
}

impl Texture {
    pub fn new(file_path: &str, sprite_size: usize) -> Self {
        let image = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = image.width();
        let height = image.height();

        let size = width * height;
        let mut colors = vec![0xffffff.into(); size as usize];

        // If I use flatmap and all that this get's reordered...
        // I don't know why
        for x in 0..width {
            for y in 0..height {
                let pixel = image.get_pixel(x, y).to_rgb();
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                let idx = y * width + x;
                colors[idx as usize] = Color { r, g, b };
            }
        }

        Texture {
            width,
            height,
            colors,
            sprite_size,
        }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> Color {
        let idx = y * self.width + x;
        self.colors[idx as usize]
    }

    /// This function assumes the sprite is configured
    /// to be read like a cloth put over the cube-like shape.
    pub fn get_color_of_face(&self, face: &CubeFace, x: f32, y: f32) -> Color {
        let sprite_size = self.sprite_size as f32;
        let point = Vec2::new(x, y);
        let origin = match face {
            CubeFace::TOP => Vec2::new(sprite_size, sprite_size),
            CubeFace::BOTTOM => Vec2::new(sprite_size, sprite_size * 3.0),
            CubeFace::FORWARDS => Vec2::new(sprite_size, sprite_size * 0.0),
            CubeFace::BACKWARDS => Vec2::new(sprite_size, sprite_size * 2.0),
            CubeFace::LEFT => Vec2::new(0.0, sprite_size),
            CubeFace::RIGHT => Vec2::new(sprite_size * 2.0, sprite_size),
            CubeFace::NONE => return 0xff00ff.into(),
        };

        let point = origin + point;
        let x = point.x.clamp(origin.x, origin.x + sprite_size - 1.0) as u32;
        let y = point.y.clamp(origin.y, origin.y + sprite_size - 1.0) as u32;

        

        self.get_pixel_color(x, y)
    }
}
