use crate::hit::HitOrMiss::Miss;
use crate::hit::{HitOrMiss, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A Sphere defined with a center point and a radius.
pub struct Sphere {
    pub center: Vec3,
    pub r: f64,
}

impl Hittable for Sphere {
    /// Test whether a given ray hit the sphere between times t_min and t_max.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitOrMiss {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm();
        let h = oc.dot(&ray.direction);
        let c = oc.norm() - self.r * self.r;
        let discriminant = h * h - a * c;
        return if discriminant < 0.0 {
            Miss
        } else {
            let sqrt_d = discriminant.sqrt();

            let root = (-h - sqrt_d) / a;
            if root < t_min || t_max < root {
                return Miss;
            }
            let p = ray.at(root);
            HitOrMiss::hit(p, (p - self.center) / self.r, root, ray)
        };
    }
}
