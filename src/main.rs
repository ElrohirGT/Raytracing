use minifb::{Key, KeyRepeat, Window, WindowOptions};
use mouse_rs::Mouse;
use nalgebra_glm::Vec3;
use rayon::iter::ParallelIterator;
use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::cube::Cube;
use raytracer::framebuffer;
use raytracer::light::Light;
use raytracer::material::{Material, STONE, WATER};
use raytracer::render::{init_render, render};
use raytracer::texture::{GameTextures, Textures};
use raytracer::{Message, Model};
use std::collections::VecDeque;
use std::env;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

const PLAYER_SPEED: f32 = 0.1;
const PLAYER_ROTATION_SPEED: f32 = PI / 20.0;

fn main() {
    let window_width = 1080;
    let window_height = 720;

    let framebuffer_width = 1080;
    let framebuffer_height = 720;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let window_options = WindowOptions {
        resize: true,
        scale: minifb::Scale::FitScreen,
        ..WindowOptions::default()
    };

    let title_prefix = "TortrixCraft RTX - ON";
    let mut window =
        Window::new(title_prefix, window_width, window_height, window_options).unwrap();
    window.set_key_repeat_delay(0.01);
    window.set_cursor_visibility(true);
    let mouse = Mouse::new();

    let target_framerate = 60;
    let frame_delay = Duration::from_millis(1000 / target_framerate);

    let mut data = init(framebuffer_width, framebuffer_height);
    init_render(&mut framebuffer, &data);

    let mut splash_timer = 0;
    let splash_delay = 300;

    let mode_cooldown = 5;
    let mut mode_cooldown_timer = 0;

    let last_recorded_frames_max_count = 60;
    let mut last_recorded_frames = VecDeque::with_capacity(last_recorded_frames_max_count);
    while window.is_open() {
        let start = Instant::now();
        mode_cooldown_timer = (mode_cooldown_timer - 1).max(0);
        splash_timer = (splash_timer + 1).min(splash_delay + 1);

        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        let messages: Vec<Message> = window
            .get_keys_pressed(KeyRepeat::Yes)
            .into_iter()
            .filter_map(|key| match key {
                Key::Left => Some(Message::RotateCamera(PLAYER_ROTATION_SPEED, 0.0)),
                Key::Right => Some(Message::RotateCamera(-PLAYER_ROTATION_SPEED, 0.0)),
                Key::Up => Some(Message::RotateCamera(0.0, -PLAYER_ROTATION_SPEED)),
                Key::Down => Some(Message::RotateCamera(0.0, PLAYER_ROTATION_SPEED)),

                Key::W => Some(Message::ZoomCamera(PLAYER_SPEED)),
                Key::S => Some(Message::ZoomCamera(-PLAYER_SPEED)),

                // Key::Space => match (mode_cooldown_timer, &data.status) {
                //     (0, GameStatus::MainMenu) => {
                //         mode_cooldown_timer = mode_cooldown;
                //         Some(Message::StartGame)
                //     }
                //     _ => None,
                // },
                // Key::R => match (mode_cooldown_timer, &data.status) {
                //     (0, GameStatus::YouLost) | (0, GameStatus::YouWon) => {
                //         mode_cooldown_timer = mode_cooldown;
                //         Some(Message::RestartGame)
                //     }
                //     _ => None,
                // },
                _ => None,
            })
            .collect();

        for msg in messages {
            data = update(data, msg);
        }

        if data.camera.has_changed() {
            render(&mut framebuffer, &data);
        }
        data.camera.reset_change();

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .expect("Couldn't update the framebuffer!");
        let end = Instant::now();
        if last_recorded_frames.len() == last_recorded_frames_max_count {
            last_recorded_frames.pop_front();
        }
        last_recorded_frames.push_back((end - start).as_millis());

        let avg_millis: f32 = last_recorded_frames.iter().map(|&u| u as f32).sum::<f32>()
            / last_recorded_frames_max_count as f32;
        let avg_frames = 1000.0 / avg_millis;
        window.set_title(format!("{} - {:.2} fps", title_prefix, avg_frames).as_ref());
        std::thread::sleep(frame_delay);
    }
}

/// Init the default state
fn init(framebuffer_width: usize, framebuffer_height: usize) -> Model {
    let mut args = env::args();
    args.next();

    let asset_dir = args.next().expect("No asset directory received!");
    println!("Reading assets from: {asset_dir}");

    // let rubber = Material {
    //     diffuse: 0xffff00.into(),
    //     specular: 1.0,
    //     albedo: (0.9, 0.1),
    //     reflectivity: 0.0,
    //     transparency: 0.0,
    //     refractive_index: 1.51,
    //     texture: Some(Textures::DIRT),
    // };

    // let ivory = Material {
    //     diffuse: 0xff00ff.into(),
    //     specular: 50.0,
    //     albedo: (0.7, 0.3),
    //     reflectivity: 0.5,
    //     transparency: 0.1,
    //     refractive_index: 1.0,
    //     texture: None,
    // };

    let spheres = vec![];

    let object_id = 0;
    let p_width_height = 4;
    let cube_size = 1.5;
    let gap = -1e-3;

    let mut cubes = generate_platform(
        object_id,
        Vec3::new(0.0, -cube_size * 3.0, 0.0),
        p_width_height,
        gap,
        cube_size,
        STONE,
    );
    let object_id = cubes.len() + 1;

    let mut water_cubes = generate_platform(
        object_id as u32,
        Vec3::new(0.0, -cube_size * 2.0, 0.0),
        p_width_height,
        gap,
        cube_size,
        WATER,
    );

    cubes.append(&mut water_cubes);

    // let cubes = vec![
    //     Cube::new(
    //         1,
    //         Vec3::new(0.0, 0.0, 0.0),
    //         2.5,
    //         dirt.clone(),
    //         Vec3::new(0.0, 0.0, 1.0).normalize(),
    //     ),
    //     Cube::new(
    //         2,
    //         Vec3::new(0.0, 0.0, -2.6),
    //         2.5,
    //         dirt.clone(),
    //         Vec3::new(0.0, 0.0, 1.0).normalize(),
    //     ),
    // ];

    println!("Cubes created: {cubes:#?}");

    let lights = vec![
        Light {
            position: Vec3::new(0.0, 10.0, -8.2),
            color: Color::white(),
            intensity: 1.0,
        },
        // Light {
        //     position: Vec3::new(-2.0, 10.0, -2.0),
        //     color: Color::white(),
        //     intensity: 2.0,
        // },
    ];

    let ambient_light = 0.20;

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let textures = GameTextures::new(&asset_dir);

    Model {
        spheres,
        cubes,
        camera,
        lights,
        ambient_light,
        textures,
    }
}

fn update(data: Model, msg: Message) -> Model {
    match msg {
        Message::RotateCamera(delta_yaw, delta_pitch) => {
            let Model { mut camera, .. } = data;

            camera.rotate_cam(delta_yaw, delta_pitch);

            Model { camera, ..data }
        }
        Message::ZoomCamera(delta_zoom) => {
            let Model { mut camera, .. } = data;

            camera.zoom_cam(delta_zoom);

            Model { camera, ..data }
        }
        Message::MoveFocus(delta_pos) => {
            let Model { mut camera, .. } = data;
            camera.move_focus(delta_pos);
            Model { camera, ..data }
        }
    }
}

fn generate_platform(
    id_count: u32,
    center: Vec3,
    size: u16,
    gap: f32,
    cube_size: f32,
    material: Material,
) -> Vec<Cube> {
    let mut object_id = id_count;
    let half_size: i32 = (size / 2) as i32;
    (-half_size..half_size)
        .map(|z| z as f32 * (cube_size + gap))
        .flat_map(|z| {
            let material = material.clone();
            (-half_size..half_size)
                .map(|x| x as f32 * (cube_size + gap))
                .map(move |x| {
                    let id = object_id;
                    object_id += 1;

                    Cube::new(
                        id,
                        Vec3::new(x, 0.0, z) + center,
                        cube_size,
                        material.clone(),
                        Vec3::new(0.0, 0.0, 1.0).normalize(),
                    )
                })
        })
        .collect()
}
