use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Option<Point3>,
    pub normal: Option<Vec3>,
    pub t: Option<f64>,
}

pub trait Hittable {
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
