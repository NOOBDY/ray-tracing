mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod random;
mod ray;

use std::rc::Rc;

use camera::Camera;
use cgmath::vec3;
use hittable::{hittable_list::HittableList, sphere::Sphere};
use material::{lambertian::Lambertian, metal::Metal};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 720;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() -> anyhow::Result<()> {
    let mat_ground = Rc::new(Lambertian {
        albedo: vec3(0.8, 0.8, 0.0),
    });
    let mat_center = Rc::new(Lambertian {
        albedo: vec3(0.7, 0.3, 0.3),
    });
    let mat_left = Rc::new(Metal {
        albedo: vec3(0.8, 0.8, 0.8),
    });
    let mat_right = Rc::new(Metal {
        albedo: vec3(0.8, 0.6, 0.2),
    });

    let world = Rc::new(HittableList::new(vec![
        Rc::new(Sphere {
            center: vec3(-1.2, 0.0, -1.0),
            radius: 0.5,
            material: mat_left,
        }),
        Rc::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: mat_center,
        }),
        Rc::new(Sphere {
            center: vec3(1.2, 0.0, -1.0),
            radius: 0.5,
            material: mat_right,
        }),
        Rc::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: mat_ground,
        }),
    ]));

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);

    cam.render(world);

    Ok(())
}
