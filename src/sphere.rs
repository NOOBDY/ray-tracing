use std::rc::Rc;

use cgmath::{dot, vec3, Vector3};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let half_b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: Rc::clone(&self.material),
            front_face: Default::default(),
            normal: vec3(0.0, 0.0, 0.0),
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
