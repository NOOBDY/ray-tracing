use crate::{
    color::Color, hittable::HitRecord, material::Material, math::near_zero,
    random::random_unit_vector, ray::Ray,
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
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
