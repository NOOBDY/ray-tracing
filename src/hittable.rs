use std::rc::Rc;

use cgmath::{dot, Vector3};

use crate::{interval::Interval, material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
