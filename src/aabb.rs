// aabb.rs

// Axis-Aligned Bounding Box (AABB) implementation (more or less good enough)

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

    fn pad_to_minimum(&mut self){
        let delta: f64 = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
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

        if x_size > y_size && x_size > z_size {
            0
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig:Point3 = r.origin();
        let ray_dir:Vec3 = r.direction();

        for axis in 0..3{
            let axis_interval: &Interval = self.axis_interval(axis);
            let adinV: f64 = 1.0 / ray_dir[axis as usize];

            // compute t0 and t1 for the slabs
            let t0 = (axis_interval.min - ray_orig[axis as usize]) * adinV;
            let t1 = (axis_interval.max - ray_orig[axis as usize]) * adinV;

            if t0 < t1{
                if t0 > ray_t.min {ray_t.min = t0;}
                if t1 < ray_t.max {ray_t.max = t1;}
            } else {
                if t1 > ray_t.min {ray_t.min = t1;}
                if t0 < ray_t.max {ray_t.max = t0;}
            }

            if ray_t.max <= ray_t.min{
                return false;
            }
        }
        return true;
    }    
}
