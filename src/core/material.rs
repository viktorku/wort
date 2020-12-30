use enum_iterator::IntoEnumIterator;
use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr, ToString};
use rand::random;

use crate::core::{color::Color, hit::HitRecord, ray::Ray, vec3::Vec3};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    IntoEnumIterator,
    ToString,
)]
#[strum(serialize_all = "kebab_case")]
pub enum DiffuseMethod {
    Simple,
    Lambert,
    Hemisphere,
}

#[derive(Debug, Clone, Copy)]
pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
    pub diffuse_method: DiffuseMethod,
}

impl Lambertian {
    pub fn new(albedo: Color, diffuse_method: DiffuseMethod) -> Lambertian {
        Lambertian {
            albedo,
            diffuse_method,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = {
            let candidate = match self.diffuse_method {
                DiffuseMethod::Hemisphere => Vec3::random_in_hemisphere(&rec.normal),
                DiffuseMethod::Simple => rec.normal + Vec3::random_in_unit_sphere(),
                DiffuseMethod::Lambert => rec.normal + Vec3::random_unit_vector(),
            };
            // Catch degenerate scatter direction
            if candidate.near_zero() {
                rec.normal
            } else {
                candidate
            }
        };
        Some(Scatter {
            ray: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = ray_in.direction.normalize().reflect(&rec.normal);
        let ray = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if ray.direction.dot(rec.normal) > 0. {
            Some(Scatter { ray, attenuation })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    /// Index of refraction:
    ///  - Air: 1.0
    ///  - Glass: 1.3 - 1.7
    ///  - Diamond: 2.4
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new_hex(b"#FFFFFF");
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction.normalize();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>()
        {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        Some(Scatter {
            attenuation,
            ray: Ray::new(rec.p, direction),
        })
    }
}
