// perlin.rs

use crate::{random_i32_range};
use crate::utils::prelude::{random_f64, random_f64_range};
use crate::vec3::{Vec3, Point3};

pub struct Perlin {
    pub point_count: usize,
    randvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: usize = 256;

        // Fill randfloat correctly (push, or collect)
        let mut randvec = Vec::with_capacity(point_count);
        for _ in 0..point_count {
            randvec.push(Vec3::new(random_f64_range(-1.0, 1.0), random_f64_range(-1.0, 1.0), random_f64_range(-1.0, 1.0)));
        }

        // Generate permutation arrays
        let perm_x = Perlin::perlin_generate_perm(point_count);
        let perm_y = Perlin::perlin_generate_perm(point_count);
        let perm_z = Perlin::perlin_generate_perm(point_count);

        Perlin {
            point_count,
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    // Create a permutation [0,1,2,...,point_count-1] and shuffle it
    fn perlin_generate_perm(point_count: usize) -> Vec<i32> {
        let mut p: Vec<i32> = (0..point_count as i32).collect();
        Perlin::permute(&mut p);
        p
    }

    // Fisher–Yates shuffle
    fn permute(p: &mut [i32]) {
        for i in (1..p.len()).rev() {
            let target = random_i32_range(0, (i + 1) as i32) as usize;
            p.swap(i, target);
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        // Classic "value noise" hash: use integer lattice coords
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();


        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let c = &mut [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[
                        (self.perm_x[((i as usize + di) & 255) as usize] ^
                         self.perm_y[((j as usize + dj) & 255) as usize] ^
                         self.perm_z[((k as usize + dk) & 255) as usize]) as usize
                    ];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)

        
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                             (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                             (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) *
                             c[i][j][k].dot(weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }
}
