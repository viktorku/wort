use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Option<Point3>,
    pub normal: Option<Vec3>,
    pub t: Option<f64>,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // the ray is hitting the front face if the normal is pointing
        //  in the opposite direction of the ray
        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = Some(if self.front_face { outward_normal } else { -outward_normal });
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
