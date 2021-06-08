use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Vec3 {
        Vec3 { x: v, y: v, z: v }
    }

    pub fn magnitude(&self) -> f64 {
        self.norm().sqrt()
    }
    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        let inv_mag = self.magnitude().recip();
        Vec3 {
            x: self.x * inv_mag,
            y: self.y * inv_mag,
            z: self.z * inv_mag,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.magnitude().powi(2) >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn test_can_add_vectors() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vec3 {
        x: 11.0,
        y: 12.0,
        z: 13.0,
    };

    assert_eq!(
        Vec3 {
            x: 12.0,
            y: 14.0,
            z: 16.0,
        },
        v1 + v2
    );
    assert_eq!(
        Vec3 {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        },
        -v1
    );
}

#[test]
fn test_can_negate_vectors() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(
        Vec3 {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        },
        -v1
    );
}
