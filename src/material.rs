use crate::ray::Ray;
use crate::vec3::Vec3;

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
    fn scatter(&self, ray_in: &Ray, normal: Vec3) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn make(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, normal: Vec3) -> ScatterResult {
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

pub struct UniformScatterer {
    albedo: Vec3,
}

impl UniformScatterer {
    pub fn make(albedo: Vec3) -> UniformScatterer {
        UniformScatterer { albedo }
    }
}

impl Material for UniformScatterer {
    fn scatter(&self, _ray_in: &Ray, normal: Vec3) -> ScatterResult {
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
    fn scatter(&self, ray_in: &Ray, normal: Vec3) -> ScatterResult {
        let v = ray_in.direction.normalize();
        // Vector calculation for getting the reflected ray direction.
        // Plus the random fuzz of the material
        let scatter_direction =
            v - 2.0 * v.dot(&normal) * normal + (self.fuzz * Vec3::random_in_unit_sphere());
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
