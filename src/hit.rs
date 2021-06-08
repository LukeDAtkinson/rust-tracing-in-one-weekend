use crate::hit::HitOrMiss::{Hit, Miss};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum HitOrMiss {
    Hit {
        p: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
    },
    Miss,
}

impl HitOrMiss {
    pub fn hit(p: Vec3, outward_normal: Vec3, t: f64, ray: &Ray) -> HitOrMiss {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Hit {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitOrMiss;
}

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitOrMiss {
        // Start assuming we are missing.
        let mut hit_or_miss = Miss;
        // Iterate over all of the hittable objects
        for hittable in &self.hittables {
            let tmp = hittable.hit(
                ray,
                t_min,
                match hit_or_miss {
                    // Only hits if the hittable is closer to the camera than an already-hit object
                    HitOrMiss::Hit { t, .. } => t,
                    // We don't have a hit yet, so a hit counts if within t_max
                    Miss => t_max,
                },
            );
            if let HitOrMiss::Hit { .. } = tmp {
                hit_or_miss = tmp;
            }
        }
        hit_or_miss
    }
}
