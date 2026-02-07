// vec3.rs
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use crate::utils::prelude::{random_f64, random_f64_range};

// 3D vector struct used to define points and colors
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

// type alias for 3D points and colors
pub type Point3 = Vec3; // 3D point
pub type Color = Vec3;  // RGB color


impl Vec3 {

    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn init_zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }
    
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }


    pub fn near_zero(&self) -> bool {
        let s = f64::MIN;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    // horrible function to generate random unit vector
    #[inline]
    pub fn random_unit_vector() -> Vec3{
        loop{
            let p = Vec3::random_range(-1.0, 1.0);
            let lensqp = p.length_squared();
            if f64::MIN < lensqp && lensqp <= 1.0 { 
                return p / lensqp.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_f64_range(-1.0, 1.0),
                random_f64_range(-1.0, 1.0),
                0.0,
            );

            // Rejection sampling: keep points uniformly sampled in the square [-1,1]×[-1,1]
            // and accept only those whose squared radius is < 1 (inside the unit disk).
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }


    #[inline]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2.0 * v.dot(n)
    }

    #[inline]
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta: f64 = (-*uv).dot(*n).min(1.0);
        let r_out_perp: Vec3 = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel: Vec3 = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        return r_out_perp + r_out_parallel;
    }


    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f64(),
            y: random_f64(),
            z: random_f64(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_f64_range(min, max),
            y: random_f64_range(min, max),
            z: random_f64_range(min, max),
        }
    }

}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

//element-wise multiplication
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


// Scalar multiplication for any scalar type that can convert into f64
// Note: only Vec3 * scalar and not the other way around (kinda dumb but whatever)
impl<T> Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn mul(self, t: T) -> Vec3 {
        let t = t.into();
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}



// Scalar multiplication end

// Div vec3 by vec3
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// Div vec3 by any scalar type that can convert into f64
impl<T> Div<T> for Vec3
where T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn div(self, s: T) -> Vec3 {
        let s = s.into();
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

// indexing for Vec3

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bounds: {}", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds: {}", i),
        }
    }
}
