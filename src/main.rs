mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;

use std::rc::Rc;

use camera::Camera;
use cgmath::vec3;

use crate::{hittable_list::HittableList, sphere::Sphere};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1080;
const SAMPLES_PER_PIXEL: u32 = 10;

fn main() -> anyhow::Result<()> {
    let world = Rc::new(HittableList::new(vec![
        Rc::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Rc::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ]));

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL);

    cam.render(world);

    Ok(())
}
