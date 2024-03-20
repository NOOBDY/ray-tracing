use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Arc<dyn Hittable>>) -> HittableList {
        HittableList { objects: list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = HitRecord {
            ..Default::default()
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            ) {
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
