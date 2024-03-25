mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod random;
mod ray;
mod sphere;

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use camera::Camera;
use cgmath::vec3;

use crate::{hittable::Hittable, hittable_list::HittableList, sphere::Sphere};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

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

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);

    static world: Arc<Mutex<Box<dyn Hittable>>> = Arc::new(Mutex::new(world));

    cam.render(world);

    Ok(())
}
