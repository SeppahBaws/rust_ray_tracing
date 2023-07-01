use rand::Rng;
use std::io::Write;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use materials::{Dielectric, Lambertian, Metal};
use objects::{MovingSphere, Sphere};
use output_buffer::OutputBuffer;
use ray::Ray;
use utils::random;
use utils::INFINITY;
use vec3::{Color, Point3, Vec3};

use crate::bvh::BVHNode;

mod aabb;
mod bvh;
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

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

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

    let begin = SystemTime::now();

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

    let end = SystemTime::now().duration_since(begin).unwrap().as_secs();

    let now = chrono::Local::now();
    println!("\rFinished rendering at {} - took {}s", now.format("%H:%M:%S"), end);
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(
                (a as f32) + 0.9 * random(),
                0.2,
                (b as f32) + 0.9 * random(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    let center2 =
                        center + Vec3::new(0.0, rand::thread_rng().gen_range(0.0..0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    world.add(Rc::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                } else {
                    // Glass
                    world.add(Rc::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    )));

    // world
    HittableList::from(Rc::new(bvh::BVHNode::from(&world, 0.0, 1.0)))
}
