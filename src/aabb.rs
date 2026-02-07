// src/aabb.rs

use crate::vec3::{Point3, Vec3};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn empty() -> Self {
        AABB {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }

    fn pad_to_minimum(&mut self) {
        let delta: f64 = 0.0001;
        if self.x.size() < delta { self.x = self.x.expand(delta); }
        if self.y.size() < delta { self.y = self.y.expand(delta); }
        if self.z.size() < delta { self.z = self.z.expand(delta); }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = AABB { x, y, z };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn extrema_box(p1: Point3, p2: Point3) -> Self {
        let mut aabb = AABB {
            x: Interval::new(p1.x().min(p2.x()), p1.x().max(p2.x())),
            y: Interval::new(p1.y().min(p2.y()), p1.y().max(p2.y())),
            z: Interval::new(p1.z().min(p2.z()), p1.z().max(p2.z())),
        };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn from_two_boxes(box1: AABB, box2: AABB) -> Self {
        let mut aabb = AABB {
            x: Interval::interval(box1.x, box2.x),
            y: Interval::interval(box1.y, box2.y),
            z: Interval::interval(box1.z, box2.z),
        };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn shift(&self, delta: Vec3) -> Self {
        AABB {
            x: self.x.shift(delta.x()),
            y: self.y.shift(delta.y()),
            z: self.z.shift(delta.z()),
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Axis index out of bounds"),
        }
    }

    pub fn longest_axis(&self) -> i32 {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();
        if x_size > y_size && x_size > z_size { 0 } else if y_size > z_size { 1 } else { 2 }
    }

    // Optimized hit function: Precomputed inverse + Unrolled loop + ORIGINAL LOGIC
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let inv_dir = r.inv_direction(); // Use precomputed value

        // Unrolled Axis 0 (X)
        let t0_0 = (self.x.min - ray_orig.x()) * inv_dir.x();
        let t1_0 = (self.x.max - ray_orig.x()) * inv_dir.x();
        
        // Exact same logic structure as your working code
        if t0_0 < t1_0 {
            if t0_0 > ray_t.min { ray_t.min = t0_0; }
            if t1_0 < ray_t.max { ray_t.max = t1_0; }
        } else {
            if t1_0 > ray_t.min { ray_t.min = t1_0; }
            if t0_0 < ray_t.max { ray_t.max = t0_0; }
        }
        if ray_t.max <= ray_t.min { return false; }

        // Unrolled Axis 1 (Y)
        let t0_1 = (self.y.min - ray_orig.y()) * inv_dir.y();
        let t1_1 = (self.y.max - ray_orig.y()) * inv_dir.y();
        
        if t0_1 < t1_1 {
            if t0_1 > ray_t.min { ray_t.min = t0_1; }
            if t1_1 < ray_t.max { ray_t.max = t1_1; }
        } else {
            if t1_1 > ray_t.min { ray_t.min = t1_1; }
            if t0_1 < ray_t.max { ray_t.max = t0_1; }
        }
        if ray_t.max <= ray_t.min { return false; }

        // Unrolled Axis 2 (Z)
        let t0_2 = (self.z.min - ray_orig.z()) * inv_dir.z();
        let t1_2 = (self.z.max - ray_orig.z()) * inv_dir.z();
        
        if t0_2 < t1_2 {
            if t0_2 > ray_t.min { ray_t.min = t0_2; }
            if t1_2 < ray_t.max { ray_t.max = t1_2; }
        } else {
            if t1_2 > ray_t.min { ray_t.min = t1_2; }
            if t0_2 < ray_t.max { ray_t.max = t0_2; }
        }

        return ray_t.max > ray_t.min;
    }    
}
