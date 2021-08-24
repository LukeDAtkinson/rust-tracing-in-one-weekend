use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn make(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, normal: Vec3, _front_face: bool) -> ScatterResult {
        let mut scatter_direction = normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        ScatterResult::Scattered {
            scatter_direction,
            attenuation: self.albedo,
        }
    }
}
