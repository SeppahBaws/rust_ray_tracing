use crate::output_buffer::OutputBuffer;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

mod hittable;
mod objects;
mod output_buffer;
mod ray;
mod vec3;

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const NR_CHANNELS: u32 = 3;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut buffer = OutputBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT, NR_CHANNELS);

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            buffer.write_color(i, j, &pixel_color);
        }
    }

    println!("\nFinished rendering. Outputting to file...");

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

fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f32::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::from(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

// TODO: continue from 6.5
// https://raytracing.github.io/books/RayTracingInOneWeekend.html#surfacenormalsandmultipleobjects/alistofhittableobjects
