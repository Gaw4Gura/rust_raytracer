use std::f64;
use rand::Rng;
use std::ops::{ Add, Sub, Mul, Div, Neg };

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{ x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn euclid(&self, vec: &Vec3) -> f64 {
        let dx = self.x - vec.x();
        let dy = self.y - vec.y();
        let dz = self.z - vec.z();

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit_vec(&self) -> Vec3 {
        let len = self.len();

        Vec3::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn dot(&self, vec: &Vec3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * vec.z - self.z * vec.y,
            self.z * vec.x - self.x * vec.z,
            self.x * vec.y - self.y * vec.x,
        )
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);

            if p.len_sqr() < 1.0 { return p; }
        }
    }

    pub fn random_unit_vec() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vec()
    }

    pub fn random_on_hemisphere(norm: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vec();

        if on_unit_sphere.dot(&norm) > 0.0 { return on_unit_sphere; }
        
        -on_unit_sphere
    }

    pub fn near_zero(&self) -> bool {
        self.x().abs() < f64::EPSILON &&
        self.y().abs() < f64::EPSILON &&
        self.z().abs() < f64::EPSILON
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scale: f64) -> Vec3 {
        Vec3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * vec.x,
            y: self.y * vec.y,
            z: self.z * vec.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scale: f64) -> Vec3 {
        Vec3 {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn test_dot() {
    let vec1 = Vec3::new(0.1, 0.2, 0.3);
    let vec2 = Vec3::new(0.2, 0.3, 0.4);

    assert_approx_eq!(vec1.dot(&vec2), 0.2);
}

#[test]
fn test_len_sqr() {
    let vec = Vec3::new(0.1, 0.2, 0.3);

    assert_approx_eq!(vec.len_sqr(), 0.14);
}