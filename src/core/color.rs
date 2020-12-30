use num::clamp;
use rand::{random, thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

const HEX_ERR: &str = "Invalid hex color provided";

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
    pub fn new_hex(hex: &[u8; 7]) -> Color {
        if let Ok(hex_str) = std::str::from_utf8(hex) {
            let r = u8::from_str_radix(&hex_str[1..3], 16).expect(HEX_ERR);
            let g = u8::from_str_radix(&hex_str[3..5], 16).expect(HEX_ERR);
            let b = u8::from_str_radix(&hex_str[5..7], 16).expect(HEX_ERR);
            Color::new_rgb(r, g, b)
        } else {
            panic!(HEX_ERR)
        }
    }
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::new(r as f64 / 255., g as f64 / 255., b as f64 / 255.)
    }
    pub fn sqrt(&self) -> Color {
        Color::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
    pub fn as_u8_slice(&self) -> [u8; 3] {
        [
            (255. * clamp(self.r, 0., 1.)) as u8,
            (255. * clamp(self.g, 0., 1.)) as u8,
            (255. * clamp(self.b, 0., 1.)) as u8,
        ]
    }
    pub fn random() -> Color {
        Color::new(random::<f64>(), random::<f64>(), random::<f64>())
    }
    pub fn random_limit(min: f64, max: f64) -> Color {
        let mut rng = thread_rng();
        Color::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ r: {}, g: {}, b: {} }}", self.r, self.g, self.b)
    }
}

impl std::ops::Neg for Color {
    type Output = Self;
    fn neg(self) -> Color {
        Color::new(-self.r, -self.g, -self.b)
    }
}

impl std::ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Self;
    fn sub(self, rhs: Color) -> Color {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;
    fn mul(self, t: f64) -> Color {
        Color::new(self.r * t, self.g * t, self.b * t)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl std::ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, t: f64) {
        self.r *= t;
        self.g *= t;
        self.b *= t;
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Color {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, t: f64) {
        *self *= 1. / t;
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Self;
    fn div(self, t: f64) -> Color {
        self * (1. / t)
    }
}
