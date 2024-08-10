use audio::AudioPlayer;
use texture::GameTextures;

pub mod audio;
pub mod bmp;
pub mod color;
pub mod enemies;
pub mod framebuffer;
pub mod raytracer;
pub mod render;
pub mod texture;

extern crate nalgebra_glm as glm;

pub fn are_equal(first: f32, second: f32, eps: f32) -> bool {
    (first - second).abs() <= eps
}

#[derive(PartialEq, Clone)]
pub enum BoardCell {
    Empty,
    Player,
    Goal,
    LoliBunny(enemies::LoliBunny),
    HorizontalWall,
    VerticalWall,
    PillarWall,
}

pub struct Board {
    pub cells: Vec<Vec<BoardCell>>,
    pub cell_dimensions: (f32, f32),
}

pub struct Model {
    pub board: Board,
    pub framebuffer_dimensions: (usize, usize),
    pub player: Player,
    pub textures: GameTextures,
    pub audio_player: AudioPlayer,
    pub lolibunnies: Vec<enemies::LoliBunny>,
    pub moon_phase: f32,
    pub status: GameStatus,
}

pub enum GameStatus {
    SplashScreen,
    MainMenu,
    Gaming,
    YouLost,
    YouWon,
}

pub struct Player {
    pub position: nalgebra_glm::Vec2,
    pub orientation: f32,
    pub fov: f32,
}

pub enum Message {
    Move(nalgebra_glm::Vec2),
    Rotate(f32),
    TickMoon,
    EndSplash,
    RestartGame,
    YouWon,
    YouLost,
    StartGame,
}
