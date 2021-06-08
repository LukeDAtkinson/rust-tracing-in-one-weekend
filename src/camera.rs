use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn camera(
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
        origin: Vec3,
    ) -> Camera {
        let width = aspect_ratio * viewport_height;
        let horizontal = Vec3 {
            x: width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height as f64,
            z: 0.0,
        };
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: focal_length,
                },
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
