mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod math;
mod random;
mod ray;

use std::rc::Rc;

use camera::Camera;
use cgmath::vec3;
use hittable::{hittable_list::HittableList, sphere::Sphere};
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 540;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() -> anyhow::Result<()> {
    let mat_ground = Rc::new(Lambertian::new(vec3(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(vec3(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(vec3(0.8, 0.6, 0.2), 0.0));

    let world = Rc::new(HittableList::new(vec![
        Rc::new(Sphere {
            center: vec3(-1.2, 0.0, -1.0),
            radius: -0.4,
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

    let cam = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        60.0,
    );

    cam.render(world);

    Ok(())
}
