use std::rc::Rc;

use cgmath::{prelude::*, vec3, Vector3};
use image::{ImageBuffer, Rgb};
use itertools::Itertools;

use crate::{
    color::{convert_color, Color},
    hittable::Hittable,
    interval::Interval,
    random::random_on_hemisphere,
    ray::Ray,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    center: Vector3<f64>,
    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let center = vec3(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64) / (image_height as f64);

        let viewport_u = vec3(viewport_width, 0.0, 0.0);
        let viewport_v = vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width.into();
        let pixel_delta_v = viewport_v / image_height.into();

        let viewport_upper_left =
            center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: Rc<dyn Hittable>) {
        let output = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .flat_map(|(j, i)| {
                print!(
                    "\r{} / {}",
                    j * self.image_width + i + 1,
                    self.image_width * self.image_height
                );

                let pixel_color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let world = Rc::clone(&world);
                        let r = self.get_ray(i, j);
                        Camera::ray_color(r, self.max_depth, world)
                    })
                    .sum();
                convert_color(pixel_color, self.samples_per_pixel)
            })
            .collect::<Vec<_>>();

        let image =
            ImageBuffer::<Rgb<u8>, _>::from_raw(self.image_width, self.image_height, &output[..])
                .unwrap();

        image.save("image.png").unwrap();
    }

    fn ray_color(r: Ray, depth: u32, world: Rc<dyn Hittable>) -> Color {
        if depth == 0 {
            return vec3(0.0, 0.0, 0.0);
        }

        let interval = Interval {
            min: 0.0,
            max: f64::INFINITY,
        };
        match world.hit(&r, interval) {
            Some(rec) => {
                // 0.5 * (rec.normal + vec3(1.0, 1.0, 1.0))
                let direction = random_on_hemisphere(rec.normal);
                let r = Ray {
                    origin: rec.p,
                    direction,
                };
                0.5 * Camera::ray_color(r, depth - 1, world)
            }
            None => {
                let unit_direction = r.direction.normalize();
                let a = 0.5 * (unit_direction.y + 1.0);

                (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0)
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc
            + (f64::from(i) * self.pixel_delta_u)
            + (f64::from(j) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            origin: ray_origin,
            direction: ray_direction,
        }
    }

    fn pixel_sample_square(&self) -> Vector3<f64> {
        let px = -0.5 * rand::random::<f64>();
        let py = -0.5 * rand::random::<f64>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
}
