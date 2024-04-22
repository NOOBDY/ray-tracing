use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
