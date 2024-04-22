use cgmath::{AbsDiffEq, Vector3};

use crate::{
    color::Color, hittable::HitRecord, material::Material, random::random_unit_vector, ray::Ray,
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

fn near_zero(v: &Vector3<f64>) -> bool {
    let s = 1e-8;
    v.x.abs_diff_eq(&0.0, s) && v.y.abs_diff_eq(&0.0, s) && v.z.abs_diff_eq(&0.0, s)
}
