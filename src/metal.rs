use cgmath::{dot, InnerSpace, Vector3};

use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray};

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal);

        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };

        let attenuation = self.albedo;

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * dot(*v, *n) * n
}
