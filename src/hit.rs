use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

#[inline]
pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    // the ray is hitting the front face if the normal is pointing
    // in the opposite direction of the ray
    let front_face = ray.direction.dot(outward_normal) < 0.;
    let normal = if front_face { outward_normal } else { -outward_normal };
    (front_face, normal)
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
