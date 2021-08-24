use crate::hit::HitOrMiss::{Hit, Miss};
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Whether a ray hit a Hittable, or missed.
pub enum HitOrMiss {
    Hit {
        p: Vec3,
        normal: Vec3,
        scatter_result: ScatterResult,
        t: f64,
        front_face: bool,
    },
    Miss,
}

impl HitOrMiss {
    /// Create a HitOrMiss::Hit for a ray hitting an object at point p, with a given outward normal
    /// at time step t.
    ///
    /// This method handles detecting whether the ray is hitting the front face of the object or
    /// not.
    pub fn hit(
        p: Vec3,
        outward_normal: Vec3,
        t: f64,
        ray: &Ray,
        material: &dyn Material,
    ) -> HitOrMiss {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let scatter_result = material.scatter(ray, normal);
        Hit {
            p,
            normal,
            t,
            front_face,
            scatter_result,
        }
    }
}

/// An object that might be hit by a ray.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitOrMiss;
}

/// A List of Hittable structs that is itself Hittable.
pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    /// Test whether the ray hits any object in the HittableList. Returns a HitOrMiss::Hit
    /// containing the details of the hit if it did. Otherwise returns a HitOrMiss::Miss.
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
