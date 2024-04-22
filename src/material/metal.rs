use cgmath::{dot, InnerSpace};

use crate::{
    color::Color, hittable::HitRecord, material::Material, math::reflect,
    random::random_in_unit_sphere, ray::Ray,
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);

        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(),
        };

        let attenuation = self.albedo;

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
