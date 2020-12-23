use crate::hit::{Hittable, HitRecord};
use crate::ray::{Ray};
use std::rc::Rc;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct HittableList<T: Hittable> {
    pub objects: Vec<Rc<T>>,
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

    pub fn add(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }

}

impl <T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t.unwrap();
            }
        }

        hit_anything
    }
}
