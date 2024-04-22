use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            ) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
