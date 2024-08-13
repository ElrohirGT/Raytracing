use hornystein::framebuffer;
use hornystein::raytracer::Material;
use hornystein::render::{init_render, render};
use hornystein::sphere::Sphere;
use hornystein::{Message, Model};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use mouse_rs::Mouse;
use nalgebra_glm::Vec3;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const PLAYER_SPEED: f32 = 3.0;
const PLAYER_ROTATION_SPEED: f32 = 0.006;

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

    let mut window =
        Window::new("Hornystein", window_width, window_height, window_options).unwrap();
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

        let mut messages: Vec<Message> = window
            .get_keys_pressed(KeyRepeat::Yes)
            .into_iter()
            .filter_map(|key| match key {
                // Key::W => {
                //     let x_delta = PLAYER_SPEED * data.player.orientation.cos();
                //     let y_delta = PLAYER_SPEED * data.player.orientation.sin();
                //     Some(Message::Move(nalgebra_glm::Vec2::new(x_delta, y_delta)))
                // }
                // Key::S => {
                //     let x_delta = PLAYER_SPEED * data.player.orientation.cos();
                //     let y_delta = PLAYER_SPEED * data.player.orientation.sin();
                //     Some(Message::Move(nalgebra_glm::Vec2::new(-x_delta, -y_delta)))
                // }
                // Key::A => Some(Message::Rotate(-PLAYER_ROTATION_SPEED * 10.0)),
                // Key::D => Some(Message::Rotate(PLAYER_ROTATION_SPEED * 10.0)),
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
        render(&mut framebuffer, &data);

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
        window.set_title(format!("Hornystein - {:.2} fps", avg_frames).as_ref());
        std::thread::sleep(frame_delay);
    }
}

/// Init the default state
fn init(framebuffer_width: usize, framebuffer_height: usize) -> Model {
    // let mut args = env::args();
    // args.next();
    //
    // let file_name = args.next().expect("No maze file name received!");
    // println!("Reading file name: {}", file_name);

    let spheres = vec![
        // Face
        Sphere {
            radius: 3.0,
            center: Vec3::new(0.0, 0.0, 0.0),
            material: Material {
                diffuse: 0xcbacd2.into(),
            },
        },
        // Left eye
        Sphere {
            radius: 1.0,
            center: Vec3::new(-1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0xffffff.into(),
            },
        },
        Sphere {
            radius: 0.8,
            center: Vec3::new(-1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0x672b8c.into(),
            },
        },
        Sphere {
            radius: 0.5,
            center: Vec3::new(-1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0x000000.into(),
            },
        },
        Sphere {
            radius: 0.05,
            center: Vec3::new(-1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0xffffff.into(),
            },
        },
        Sphere {
            radius: 0.15,
            center: Vec3::new(-1.75, 1.75, 0.0),
            material: Material {
                diffuse: 0xffffff.into(),
            },
        },
        // Right eye
        Sphere {
            radius: 1.0,
            center: Vec3::new(1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0xffffff.into(),
            },
        },
        Sphere {
            radius: 0.8,
            center: Vec3::new(1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0x672b8c.into(),
            },
        },
        Sphere {
            radius: 0.5,
            center: Vec3::new(1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0x000000.into(),
            },
        },
        Sphere {
            radius: 0.05,
            center: Vec3::new(1.5, 1.5, 0.0),
            material: Material {
                diffuse: 0xffffff.into(),
            },
        },
        Sphere {
            radius: 0.15,
            center: Vec3::new(1.75, 1.75, 0.0),
            material: Material {
                diffuse: 0xffffff.into(),
            },
        },
        // Mouth
        Sphere {
            radius: 1.0,
            center: Vec3::new(0.0, -2.0, 0.0),
            material: Material {
                diffuse: 0xdf2080.into(),
            },
        },
        Sphere {
            radius: 0.5,
            center: Vec3::new(0.0, -2.6, 0.0),
            material: Material {
                diffuse: 0xfe6d01.into(),
            },
        },
        // Right ear
        Sphere {
            radius: 1.0,
            center: Vec3::new(3.4, 3.4, 0.0),
            material: Material {
                diffuse: 0xcbacd2.into(),
            },
        },
        // Left ear
        Sphere {
            radius: 1.0,
            center: Vec3::new(-3.4, 3.4, 0.0),
            material: Material {
                diffuse: 0xcbacd2.into(),
            },
        },
    ];

    Model { spheres }
}

fn update(data: Model, msg: Message) -> Model {
    data
}
