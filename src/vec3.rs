use rand::{random, thread_rng, Rng};

pub trait Length {
    fn length(self) -> f64;
    fn length_squared(self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // w: f64
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }
    pub fn random() -> Vec3 {
        Vec3::new(random::<f64>(), random::<f64>(), random::<f64>())
    }
    pub fn random_limit(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let point = Vec3::random_limit(-1., 1.);
            if point.length_squared() < 1. {
                return point;
            }
        }
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0. {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn sqrt(&mut self) -> &Vec3 {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
        self
    }
}

impl Length for Vec3 {
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    fn length_squared(self) -> f64 {
        self.dot(self)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Vec3 {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1. / t;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Vec3 {
        self * (1. / t)
    }
}
