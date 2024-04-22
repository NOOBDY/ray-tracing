use cgmath::{dot, vec3, InnerSpace};

use crate::{
    math::{reflect, refract},
    random::random_f64,
    ray::Ray,
};

use super::Material;

pub struct Dielectric {
    pub ior: f64, // index of refraction
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric { ior }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(crate::color::Color, crate::ray::Ray)> {
        let attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        let scattered = Ray {
            origin: rec.p,
            direction,
        };

        Some((attenuation, scattered))
    }
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
