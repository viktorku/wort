use crate::hit::{Hittable, HitRecord};
use crate::ray::{Ray};
use std::sync::Arc;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct HittableList<T: Hittable> {
    pub objects: Vec<Arc<T>>,
}

impl <T: Hittable> HittableList<T> {
    pub fn new() -> HittableList<T> {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<T>) {
        self.objects.push(object);
    }

}

impl <T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
