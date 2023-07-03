use rand::Rng;
use std::rc::Rc;

use crate::{
    aarect::{XYRect, XZRect, YZRect},
    bvh,
    hittable_list::HittableList,
    materials::DiffuseLight,
    materials::{Dielectric, Lambertian, Metal},
    objects::{MovingSphere, Sphere},
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    utils::random,
    vec3::{Color, Point3, Vec3},
};

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::from(0.9)),
    );
    let ground_material = Lambertian::from_texture(Rc::new(checker));
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

pub fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::from(0.9)),
    ));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::from_texture(checker.clone()),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::from_texture(checker.clone()),
    )));

    world
}

pub fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let perlin_texture = Rc::new(NoiseTexture::new(4.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(perlin_texture.clone()),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(perlin_texture.clone()),
    )));

    world
}

pub fn earth() -> HittableList {
    let earth_texture = Rc::new(ImageTexture::new("res/earthmap.jpg"));
    let globe = Rc::new(Sphere::new(
        Point3::from(0.0),
        2.0,
        Lambertian::from_texture(earth_texture),
    ));

    HittableList::from(globe)
}

pub fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(4.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(pertext.clone()),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(pertext.clone()),
    )));

    let difflight = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    world.add(Rc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    world
}

pub fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    world.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    world.add(Rc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world
}
