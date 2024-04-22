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
use cgmath::{vec3, ElementWise, MetricSpace};
use hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use random::{random_f64, random_f64_with_min_max, random_vec3, random_vec3_with_min_max};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 720;
const SAMPLES_PER_PIXEL: u32 = 200;
const MAX_DEPTH: u32 = 50;

fn main() -> anyhow::Result<()> {
    let mat_ground = Rc::new(Lambertian::new(vec3(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(vec3(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    let mat_right = Rc::new(Metal::new(vec3(0.8, 0.6, 0.2), 0.0));

    let mut balls: Vec<Rc<dyn Hittable>> = Vec::new();

    (-11..11)
        .flat_map(|a| {
            (-11..11).filter_map(move |b| {
                let a = f64::from(a);
                let b = f64::from(b);
                let r = random_f64();

                let center = vec3(a + 0.9 * random_f64(), -0.3, b + 0.9 * random_f64());

                if (center - vec3(4.0, 0.2, 0.0)).distance(vec3(0.0, 0.0, 0.0)) > 0.9 {
                    if r < 0.8 {
                        let albedo = random_vec3().mul_element_wise(random_vec3());
                        let mat = Rc::new(Lambertian::new(albedo));
                        Some(Rc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: mat,
                        }))
                    } else if r < 0.95 {
                        let albedo = random_vec3_with_min_max(0.5, 1.0);
                        let fuzz = random_f64_with_min_max(0.0, 0.5);
                        let mat = Rc::new(Metal::new(albedo, fuzz));
                        Some(Rc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: mat,
                        }))
                    } else {
                        let mat = Rc::new(Dielectric::new(1.5));
                        Some(Rc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: mat,
                        }))
                    }
                } else {
                    None
                }
            })
        })
        .for_each(|b| {
            balls.push(b);
        });

    vec![
        Rc::new(Sphere {
            center: vec3(-1.2, 0.0, -1.0),
            radius: 0.5,
            material: mat_left,
        }),
        Rc::new(Sphere {
            center: vec3(-1.2, 0.0, -1.0),
            radius: 0.4,
            material: mat_bubble,
        }),
        Rc::new(Sphere {
            center: vec3(0.0, 0.0, -1.2),
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
    ]
    .into_iter()
    .for_each(|b| {
        balls.push(b);
    });

    let world = Rc::new(HittableList::new(balls));

    let cam = Camera::new(
        vec3(-2.0, 0.5, 1.0),
        vec3(0.0, 0.0, -1.0),
        vec3(0.0, 1.0, 0.0),
        60.0,
        2.0,
        2.6,
    );

    cam.render(world);

    Ok(())
}
