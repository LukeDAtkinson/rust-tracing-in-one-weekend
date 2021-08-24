use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod uniform_scatterer;

/// The result of scattering a ray from a Material.
pub enum ScatterResult {
    Scattered {
        scatter_direction: Vec3,
        attenuation: Vec3,
    },
    Absorbed {},
}

pub trait Material {
    /// Scatter a ray of a given attenuation that hit this Material according to the hit_record.
    /// Returns a ScatterResult.
    fn scatter(&self, ray_in: &Ray, normal: Vec3, front_face: bool) -> ScatterResult;
}

fn reflect(ray_in: &Ray, normal: Vec3, fuzz: f64) -> Vec3 {
    let v = ray_in.direction.normalize();
    // Vector calculation for getting the reflected ray direction.
    // Plus the random fuzz of the material
    let scatter_direction =
        v - 2.0 * v.dot(&normal) * normal + (fuzz * Vec3::random_in_unit_sphere());
    scatter_direction
}
