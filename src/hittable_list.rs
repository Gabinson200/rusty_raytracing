// hittable_list.rs

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::AABB;
use std::sync::Arc;
use std::mem;

pub type HittablePtr = Arc<dyn Hittable>;

pub struct HittableList {
    objects: Vec<HittablePtr>,
    bbox: Option<AABB>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new(), bbox: None }
    }

    // take Box<dyn Hittable> but internally store Arc
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        let obj: HittablePtr = Arc::from(object);

        self.bbox = Some(match self.bbox {
            None => obj.bounding_box(),
            Some(b) => AABB::from_two_boxes(b, obj.bounding_box()),
        });

        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = None;
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }

    pub fn objects(&self) -> &Vec<HittablePtr> {
        &self.objects
    }
}


impl Hittable for HittableList {
    fn hit(&self, r:&Ray, ray_t:Interval, rec:&mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(); // Temporary hit record
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                mem::swap(rec, &mut temp_rec); //*rec = temp_rec.clone(); // clone but cheap
            }
        }

        hit_anything
    }
}
