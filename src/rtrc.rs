use std::sync::Arc;

use rtrcrs::{
    camera::Camera,
    color::{anti_aliased, Color},
    definitions::random_double,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    ray::Point3,
    sphere::Sphere,
    Vec3,
};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

pub struct RayTracer {
    world: HittableList,
    camera: Camera,
}

impl RayTracer {
    pub fn default() -> Self {
        //World
        let mut world = HittableList::default();

        let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        let material1 = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));

        let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));

        // Camera
        let camera = Camera::new(
            &Point3::new(13.0, 2.0, 3.0),
            &Point3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
            20.0,
            ASPECT_RATIO,
            0.1,
            10.0,
        );

        Self { world, camera }
    }

    pub fn render(&self, i: u16, j: u16) -> Vec<u8> {
        let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
            .map(|_| {
                let (u, v) = (
                    (i as f64 + random_double(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64,
                    (j as f64 + random_double(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64,
                );
                self.camera.get_ray(u, v).color(&self.world, MAX_DEPTH)
            })
            .sum();
        anti_aliased(pixel_color, SAMPLES_PER_PIXEL)
            .iter()
            .map(|c| (c * 255.9) as u8)
            .collect::<Vec<u8>>()
    }
}
