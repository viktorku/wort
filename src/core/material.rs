use enum_iterator::IntoEnumIterator;
use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr, ToString};

use crate::core::{
    hit::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

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
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = ray_in.direction.normalize().reflect(&rec.normal);
        let ray = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if ray.direction.dot(rec.normal) > 0. {
            Some(Scatter { ray, attenuation })
        } else {
            None
        }
    }
}
