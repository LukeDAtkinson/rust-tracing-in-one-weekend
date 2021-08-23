use crate::vec3::Vec3;

/// A Ray that is shot into the scene. The Ray has a starting position ("origin") and a direction.
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Calculate the position of the ray at time step t.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
