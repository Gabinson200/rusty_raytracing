// hittable.rs

use std::sync::{Arc, OnceLock};

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, Color};
use crate::interval::Interval;
use crate::material::{Material, Lambertian, Metal};
use crate::aabb::AABB;


#[derive(Clone)]
pub struct HitRecord {
    pub p:Point3,
    pub normal:Vec3,
    pub material: Arc<dyn Material>,
    pub t:f64,
    pub front_face:bool,
    pub u: f64,
    pub v: f64,
}

fn default_material() -> Arc<dyn Material> {
    static DEFAULT: OnceLock<Arc<dyn Material>> = OnceLock::new();
    DEFAULT
        .get_or_init(|| Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))))
        .clone()
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::init_zero(),
            normal: Vec3::init_zero(),
            material: default_material(),
            t: 0.0,
            front_face: true,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, r:&Ray, outward_normal:Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r:&Ray, interval:Interval, rec:&mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB {
        AABB::empty()
    }

}


// Translation transformation for hittable objects
pub struct Translate {
    pub hittable: Arc<dyn Hittable>,
    pub offset: Vec3,
    pub bbox: AABB,
}

impl Translate {

    pub fn new(hittable: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = hittable.bounding_box();
        let offset_bbox = AABB::new(bbox.x.shift(offset.x()), bbox.y.shift(offset.y()), bbox.z.shift(offset.z()));
        Self { hittable, offset, bbox: offset_bbox }
    }

    pub fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Hittable for Translate{
    fn hit(&self, r:&Ray, interval:Interval, rec:&mut HitRecord) -> bool {
        
        // Move teh ray backwards by the offset
        let offset_r = Ray::new_time(r.origin() - self.offset, r.direction(), r.time());

        // DEtermine wether an intersection exisst alng hte offset ray and where
        if !self.hittable.hit(&offset_r, interval, rec) {
            return false;
        }

        // Move the intersection point forward by the offset
        rec.p = rec.p + self.offset;

        return true;
        
    }
}

// Rotation around the Y axis transformation for hittable objects

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY{
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        // Compute the bounding box of the rotated object
        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = if i == 0 { bbox.x.min } else { bbox.x.max };
                    let y = if j == 0 { bbox.y.min } else { bbox.y.max };
                    let z = if k == 0 { bbox.z.min } else { bbox.z.max };

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Point3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let rotated_bbox = AABB::new(Interval::new(min.x(), max.x()), Interval::new(min.y(), max.y()), Interval::new(min.z(), max.z()));

        Self { object, sin_theta, cos_theta, bbox: rotated_bbox }
    }
}


impl Hittable for RotateY {
    fn hit(&self, r:&Ray, interval:Interval, rec:&mut HitRecord) -> bool {
        // Rotate ray in the opposite direction
        let origin = r.origin();
        let direction = r.direction();

        let rotated_origin = Point3::new(
            self.cos_theta * origin.x() - self.sin_theta * origin.z(),
            origin.y(),
            self.sin_theta * origin.x() + self.cos_theta * origin.z(),
        );

        let rotated_direction = Vec3::new(
            self.cos_theta * direction.x() - self.sin_theta * direction.z(),
            direction.y(),
            self.sin_theta * direction.x() + self.cos_theta * direction.z(),
        );

        let rotated_r = Ray::new_time(rotated_origin, rotated_direction, r.time());

        if !self.object.hit(&rotated_r, interval, rec) {
            return false;
        }

        // Rotate the hit point and normal back
        let p = rec.p;
        let normal = rec.normal;

        let rotated_p = Point3::new(
            self.cos_theta * p.x() + self.sin_theta * p.z(),
            p.y(),
            -self.sin_theta * p.x() + self.cos_theta * p.z(),
        );

        let rotated_normal = Vec3::new(
            self.cos_theta * normal.x() + self.sin_theta * normal.z(),
            normal.y(),
            -self.sin_theta * normal.x() + self.cos_theta * normal.z(),
        );

        rec.p = rotated_p;
        rec.set_face_normal(&rotated_r, rotated_normal);

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
