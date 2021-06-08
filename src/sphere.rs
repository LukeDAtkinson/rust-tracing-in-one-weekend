use crate::hit::HitOrMiss::Miss;
use crate::hit::{HitOrMiss, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub r: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitOrMiss {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude().powi(2);
        let h = oc.dot(&ray.direction);
        let c = oc.magnitude().powi(2) - self.r * self.r;
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
