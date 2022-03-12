use std::io::Write;

use hittable::Hittable;
use utils::INFINITY;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::materials::{Lambertian, Metal, Dielectric};
use crate::objects::Sphere;
use crate::output_buffer::OutputBuffer;
use crate::ray::Ray;
use crate::utils::random;
use crate::vec3::{Color, Point3, Vec3};

mod camera;
mod hittable;
mod hittable_list;
mod materials;
mod objects;
mod output_buffer;
mod ray;
mod utils;
mod vec3;

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const NR_CHANNELS: u32 = 3;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // Ground
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    // Center sphere
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    // Left sphere
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    // Left inner sphere
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left.clone(),
    )));
    // Right sphere
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    let cam = Camera::new();

    let mut buffer = OutputBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT, NR_CHANNELS);

    let now = chrono::Local::now();
    println!(
        "Started render at {} - {}x{} @ {}SPP - ray depth: {}",
        now.format("%H:%M:%S"),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH
    );

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {:04}", j);
        std::io::stdout().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f32) + random()) / (IMAGE_WIDTH - 1) as f32;
                let v = ((j as f32) + random()) / (IMAGE_HEIGHT - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            buffer.write_color(i, j, &pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    let now = chrono::Local::now();
    println!("\rFinished rendering at {}", now.format("%H:%M:%S"));
    println!("Writing buffer to file...");

    if NR_CHANNELS != 3 && NR_CHANNELS != 4 {
        panic!("Incorrect amount of channels! use either 3 or 4");
    }

    let color_type = if NR_CHANNELS == 3 {
        image::ColorType::Rgb8
    } else {
        image::ColorType::Rgba8
    };

    match image::save_buffer(
        "output.png",
        &buffer.get_pixels(),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        color_type,
    ) {
        Err(what) => panic!("Something failed! {:?}", what),
        _ => println!("Wrote image to 'output.png'!"),
    }
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::from(0.0);
    }

    match world.hit(&r, 0.001, INFINITY) {
        Some(rec) => {
            let (is_scattered, attenuation, scattered) = rec.mat.scatter(r, &rec);
            if is_scattered {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            return Color::from(0.0);
        }
        _ => (),
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::from(1.0) + t * Color::new(0.5, 0.7, 1.0)
}
