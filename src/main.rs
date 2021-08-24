use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::camera::Camera;
use crate::hit::{HitOrMiss, Hittable, HittableList};
use crate::material::{Lambertian, Metal, ScatterResult, UniformScatterer};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;

#[macro_use]
mod vec3;
mod camera;
mod hit;
mod material;
mod ray;
mod sphere;

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Vec3 {
    // We have exceeded the bounce limit. No more light gathered.
    if depth <= 0 {
        return Vec3::zero();
    };
    match world.hit(ray, 0.001, f64::INFINITY) {
        HitOrMiss::Hit {
            p,
            normal: _,
            scatter_result,
            ..
        } => {
            // Normalize ensures all components in range [0.0,1.0],
            // + [1,1,1] ensures all are in range [1.0, 2.0]
            // 0.5 * => all in range [0.5,1.0]
            match scatter_result {
                ScatterResult::Scattered {
                    scatter_direction,
                    attenuation,
                } => {
                    attenuation
                        * ray_color(
                            &Ray {
                                origin: p,
                                direction: scatter_direction,
                            },
                            world,
                            depth - 1,
                        )
                }
                ScatterResult::Absorbed { .. } => Vec3::zero(),
            }
        }
        HitOrMiss::Miss => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::from_one(1.0)
                + t * Vec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                }
        }
    }
}

fn vec_to_color(color_vec: Vec3, samples_per_pixel: usize) -> Color {
    let scaled = color_vec / samples_per_pixel as f64;
    Color::RGB(
        (scaled.x.sqrt().clamp(0.0, 0.999) * 256.0) as u8,
        (scaled.y.sqrt().clamp(0.0, 0.999) * 256.0) as u8,
        (scaled.z.sqrt().clamp(0.0, 0.999) * 256.0) as u8,
    )
}

// Image
const WIDTH: i32 = 600;
const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO) as i32;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LENGTH: f64 = 1.0;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn main() {
    // Camera
    let viewport_height = 2.0;
    let camera = Camera::camera(ASPECT_RATIO, viewport_height, FOCAL_LENGTH, Vec3::zero());

    // Set up SDL to draw to screen
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("RAYS", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGBA(255, 0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Define our materials
    let material_ground = Box::from(UniformScatterer::make(Vec3 {
        x: 0.8,
        y: 0.8,
        z: 0.0,
    }));
    let material_center = Box::from(Lambertian::make(Vec3 {
        x: 0.7,
        y: 0.3,
        z: 0.3,
    }));
    let material_left = Box::from(Metal::make(
        Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        0.3,
    ));
    let material_right = Box::from(Metal::make(
        Vec3 {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        1.0,
    ));

    // Set up the game world
    let mut world = HittableList { hittables: vec![] };
    world.hittables.push(Box::from(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
        material: material_center,
    }));
    world.hittables.push(Box::from(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        r: 100.0,
        material: material_ground,
    }));
    world.hittables.push(Box::from(Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
        material: material_left,
    }));
    world.hittables.push(Box::from(Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
        material: material_right,
    }));

    // Render with ray tracing
    'render: for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let mut pixel_color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (WIDTH - 1) as f64;
                let v = ((HEIGHT - j) as f64 + random_double()) / (HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH);
            }
            canvas.set_draw_color(vec_to_color(pixel_color, SAMPLES_PER_PIXEL));
            canvas.draw_point(Point::from((i, j))).unwrap();

            // Handle user input
            // Escape key exits, everything else is ignored
            // Checking after each pixel prevents windows from saying we are not responding
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'render;
                    }
                    _ => {}
                }
            }
        }
        // Presenting the canvas after each line
        canvas.present();
    }

    'running: loop {
        // Handle user input
        // Escape key exits, everything else is ignored
        // Checking after each pixel prevents windows from saying we are not responding
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// Returns a random number in [0,1)
fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
