use crate::material::{reflect, Material, ScatterResult};
use crate::random_double;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn make(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }

    fn refract(incident_direction: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        // Trigonometric magic. Assume I definitely solved it myself and didn't blindly copy from
        // chapter 10.2 of the book. We compute the components of the refracted ray parallel and
        // perpendicular to the surface, and we add them to get the resulting vector.
        let cos_theta = (-incident_direction).dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * (incident_direction + cos_theta * normal);
        let r_out_parallel = -(((1.0 - r_out_perp.norm()).abs()).sqrt()) * normal;
        r_out_perp + r_out_parallel
    }

    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
        r0 + (1.0 - r0) * ((1.0 - cos_theta).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, normal: Vec3, front_face: bool) -> ScatterResult {
        let refraction_ratio = if front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let incident_direction = ray_in.direction.normalize();
        let cos_theta = (-incident_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let total_internal_reflection = refraction_ratio * sin_theta > 1.0;
        let scatter_direction = if total_internal_reflection
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            reflect(ray_in, normal, 0.0)
        } else {
            Dielectric::refract(incident_direction, normal, refraction_ratio)
        };
        ScatterResult::Scattered {
            scatter_direction,
            attenuation: Vec3::from_one(1.0),
        }
    }
}
