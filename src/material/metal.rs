use crate::material::{reflect, Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn make(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, normal: Vec3, _front_face: bool) -> ScatterResult {
        let scatter_direction = reflect(ray_in, normal, self.fuzz);
        // If the reflected ray is pointing out, then the ray is reflected
        if scatter_direction.dot(&normal) > 0.0 {
            ScatterResult::Scattered {
                scatter_direction,
                attenuation: self.albedo,
            }
        }
        // otherwise the ray is absorbed
        else {
            ScatterResult::Absorbed {}
        }
    }
}
