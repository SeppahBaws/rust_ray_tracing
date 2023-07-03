use std::io::Write;
use std::time::SystemTime;

use crate::{
    camera::Camera,
    hittable::Hittable,
    hittable_list::HittableList,
    output_buffer::OutputBuffer,
    ray::Ray,
    utils::random,
    utils::INFINITY,
    vec3::{Color, Point3, Vec3},
};

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod materials;
mod objects;
mod output_buffer;
mod perlin;
mod ray;
mod scenes;
mod texture;
mod utils;
mod vec3;

struct SceneInfo {
    pub world: HittableList,
    pub background: Color,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vfov: f32,
    pub aperture: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
}

impl Default for SceneInfo {
    fn default() -> Self {
        Self {
            world: HittableList::new(),
            background: Color::from(0.0),
            lookfrom: Point3::new(10.0, 2.0, 3.0),
            lookat: Point3::from(0.0),
            vfov: 20.0,
            aperture: 0.0,
            image_width: 400,
            image_height: 225,
            samples_per_pixel: 100,
        }
    }
}

fn main() {
    const NR_CHANNELS: u32 = 3;
    const MAX_DEPTH: i32 = 50;

    let scene_select = 0;
    let scene: SceneInfo = match scene_select {
        1 => SceneInfo {
            world: scenes::random_scene(),
            background: Color::new(0.7, 0.8, 1.0),
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::from(0.0),
            aperture: 0.1,
            ..Default::default()
        },
        2 => SceneInfo {
            world: scenes::two_spheres(),
            background: Color::new(0.7, 0.8, 1.0),
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::from(0.0),
            ..Default::default()
        },
        3 => SceneInfo {
            world: scenes::two_perlin_spheres(),
            background: Color::new(0.7, 0.8, 1.0),
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::from(0.0),
            ..Default::default()
        },
        4 => SceneInfo {
            world: scenes::earth(),
            background: Color::new(0.7, 0.8, 1.0),
            lookfrom: Point3::new(13.0, 2.0, 1.0),
            lookat: Point3::from(0.0),
            ..Default::default()
        },
        5 => SceneInfo {
            world: scenes::simple_light(),
            background: Color::new(0.0, 0.0, 0.0),
            lookfrom: Point3::new(26.0, 3.0, 6.0),
            lookat: Point3::new(0.0, 2.0, 0.0),
            ..Default::default()
        },
        6 | _ => SceneInfo {
            world: scenes::cornell_box(),
            background: Color::new(0.0, 0.0, 0.0),
            lookfrom: Point3::new(278.0, 278.0, -800.0),
            lookat: Point3::new(278.0, 278.0, 0.0),
            vfov: 40.0,
            image_width: 600,
            image_height: 600,
            samples_per_pixel: 200,
            ..Default::default()
        },
    };

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aspect_ratio = scene.image_width as f32 / scene.image_height as f32;

    let cam = Camera::new(
        scene.lookfrom,
        scene.lookat,
        vup,
        scene.vfov,
        aspect_ratio,
        scene.aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut buffer = OutputBuffer::new(scene.image_width, scene.image_height, NR_CHANNELS);

    let now = chrono::Local::now();
    println!(
        "Started render at {} - {}x{} @ {}SPP - ray depth: {}",
        now.format("%H:%M:%S"),
        scene.image_width,
        scene.image_height,
        scene.samples_per_pixel,
        MAX_DEPTH
    );

    let begin = SystemTime::now();

    for j in (0..scene.image_height).rev() {
        print!("\rScanlines remaining: {:04}", j);
        std::io::stdout().flush().unwrap();

        for i in 0..scene.image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..scene.samples_per_pixel {
                let u = ((i as f32) + random()) / (scene.image_width - 1) as f32;
                let v = ((j as f32) + random()) / (scene.image_height - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &scene.background, &scene.world, MAX_DEPTH);
            }

            buffer.write_color(i, j, &pixel_color, scene.samples_per_pixel);
        }
    }

    let end = SystemTime::now().duration_since(begin).unwrap().as_secs();

    let now = chrono::Local::now();
    println!(
        "\rFinished rendering at {} - took {}s",
        now.format("%H:%M:%S"),
        end
    );
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
        scene.image_width,
        scene.image_height,
        color_type,
    ) {
        Err(what) => panic!("Something failed! {:?}", what),
        _ => println!("Wrote image to 'output.png'!"),
    }
}

fn ray_color(r: &Ray, background_color: &Color, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::from(0.0);
    }

    match world.hit(&r, 0.001, INFINITY) {
        Some(rec) => {
            let (is_scattered, attenuation, scattered) = rec.mat.scatter(r, &rec);
            let emitted = rec.mat.emitted(&rec.uv, &rec.p);

            if !is_scattered {
                return emitted;
            }

            return emitted
                + attenuation * ray_color(&scattered, &background_color, world, depth - 1);
        }
        // If the ray hits nothing, return the background color
        None => return background_color.clone(),
    }
}
