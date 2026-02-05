// bvh.rs

use std::cmp::Ordering;
use std::sync::Arc;

use crate::hittable::{Hittable, HitRecord};
use crate::hittable_list::{HittableList, HittablePtr};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::AABB;
use crate::utils::prelude::random_i32_range;

pub struct BVHNode {
    left: HittablePtr,
    right: HittablePtr,
    bbox: AABB,
}

impl BVHNode {
    // Build BVH from an existing list (consumes nothing; clones Arcs)
    pub fn new(list: &HittableList) -> Self {
        let mut objects = list.objects().clone(); // clones Arcs (cheap)
        let end = objects.len();
        Self::new_interval(&mut objects, 0, end)
    }

    fn new_interval(objects: &mut [HittablePtr], start: usize, end: usize) -> Self {

        let mut bbox = AABB::empty();

        for i in start..end {
            bbox = AABB::from_two_boxes(bbox, objects[i].bounding_box());
        }

        let axis = bbox.longest_axis();

        let object_span = end - start;

        // Sort the sub-slice in-place by bbox min along chosen axis
        objects[start..end].sort_by(|a, b| {
            let a_min = a.bounding_box().axis_interval(axis).min;
            let b_min = b.bounding_box().axis_interval(axis).min;
            a_min
                .partial_cmp(&b_min)
                .unwrap_or(Ordering::Equal)
        });

        let (left, right): (HittablePtr, HittablePtr) = match object_span {
            1 => {
                let a = objects[start].clone();
                (a.clone(), a) // same leaf on both sides like the book
            }
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                let mid = start + object_span / 2;
                let left_node = Arc::new(Self::new_interval(objects, start, mid));
                let right_node = Arc::new(Self::new_interval(objects, mid, end));
                (left_node, right_node)
            }
        };

        //let bbox = AABB::from_two_boxes(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let t_max = if hit_left { rec.t } else { ray_t.max };
        let right_interval = Interval::new(ray_t.min, t_max);

        let hit_right = self.right.hit(r, right_interval, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
