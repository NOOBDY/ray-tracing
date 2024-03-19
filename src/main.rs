mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use std::{f64::INFINITY, rc::Rc};

use cgmath::{prelude::*, vec3, Vector3};
use hittable::{HitRecord, Hittable};
use image::{ImageBuffer, Rgb};
use ray::Ray;

use crate::{hittable_list::HittableList, sphere::Sphere};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 720;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

type Color = Vector3<f64>;

fn ray_color(r: Ray, world: Rc<dyn Hittable>) -> Color {
    let mut rec = HitRecord {
        ..Default::default()
    };

    if world.hit(&r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0)
}

fn convert_color(color: Color) -> [u8; 3] {
    let x = (255.999 * color.x) as u8;
    let y = (255.999 * color.y) as u8;
    let z = (255.999 * color.z) as u8;

    [x, y, z]
}

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

    let focal_length = 1.0;
    let camera_center = vec3(0.0, 0.0, 0.0);

    let viewport_u = vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = vec3(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH.into();
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT.into();

    let viewport_upper_left =
        camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let output = (0..IMAGE_HEIGHT)
        .flat_map(|j| {
            let world = Rc::clone(&world);
            (0..IMAGE_WIDTH).flat_map(move |i| {
                let pixel_center =
                    pixel00_loc + (f64::from(i) * pixel_delta_u) + (f64::from(j) * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;

                let r = Ray {
                    origin: camera_center,
                    direction: ray_direction,
                };

                let world = Rc::clone(&world);
                let pixel_color = ray_color(r, world);
                convert_color(pixel_color)
            })
        })
        .collect::<Vec<_>>();

    let image =
        ImageBuffer::<Rgb<u8>, _>::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, &output[..]).unwrap();

    image.save("image.png")?;

    println!("Done");

    Ok(())
}
