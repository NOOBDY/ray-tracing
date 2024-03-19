use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Rc<dyn Hittable>>) -> HittableList {
        HittableList { objects: list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord {
            ..Default::default()
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                temp_rec = rec;
            }
        }

        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
    }
}
