use crate::ray::Ray;
use crate::vec3::Vec3;

/// The result of scattering a ray from a Material.
pub enum ScatterResult {
    Scattered {
        scattered_ray: Ray,
        attenuation: Vec3,
    },
    Absorbed {},
}

pub trait Material {
    /// Scatter a ray of a given attenuation that hit this Material according to the hit_record.
    /// Returns a ScatterResult.
    fn scatter(&self, ray_in: &Ray, p: Vec3, normal: Vec3) -> ScatterResult;
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
    fn scatter(&self, _ray_in: &Ray, p: Vec3, normal: Vec3) -> ScatterResult {
        let mut scatter_direction = normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        let scattered = Ray {
            origin: p,
            direction: scatter_direction,
        };
        ScatterResult::Scattered {
            scattered_ray: scattered,
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
    fn scatter(&self, _ray_in: &Ray, p: Vec3, normal: Vec3) -> ScatterResult {
        let mut scatter_direction = Vec3::random_in_hemisphere(&normal);

        // Prevent scatter direction being the zero vector, which can lead to infinities/NaNs.
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        let scattered = Ray {
            origin: p,
            direction: scatter_direction,
        };
        ScatterResult::Scattered {
            scattered_ray: scattered,
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
    fn scatter(&self, ray_in: &Ray, p: Vec3, normal: Vec3) -> ScatterResult {
        let v = ray_in.direction.normalize();
        let reflected =
            v - 2.0 * v.dot(&normal) * normal + (self.fuzz * Vec3::random_in_unit_sphere());
        if reflected.dot(&normal) > 0.0 {
            let scattered = Ray {
                origin: p,
                direction: reflected,
            };
            ScatterResult::Scattered {
                scattered_ray: scattered,
                attenuation: self.albedo,
            }
        } else {
            ScatterResult::Absorbed {}
        }
    }
}
