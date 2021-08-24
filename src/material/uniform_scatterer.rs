use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct UniformScatterer {
    albedo: Vec3,
}

impl UniformScatterer {
    pub fn make(albedo: Vec3) -> UniformScatterer {
        UniformScatterer { albedo }
    }
}

impl Material for UniformScatterer {
    fn scatter(&self, _ray_in: &Ray, normal: Vec3, _front_face: bool) -> ScatterResult {
        let mut scatter_direction = Vec3::random_in_hemisphere(&normal);

        // Prevent scatter direction being the zero vector, which can lead to infinities/NaNs.
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        ScatterResult::Scattered {
            scatter_direction,
            attenuation: self.albedo,
        }
    }
}
