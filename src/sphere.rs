use crate::vec3::{Vec3, Length};
use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // (𝐀+𝑡𝐛−𝐂)⋅(𝐀+𝑡𝐛−𝐂)=𝑟2
        // 𝑡2𝐛⋅𝐛+2𝑡𝐛⋅(𝐀−𝐂)+(𝐀−𝐂)⋅(𝐀−𝐂)−𝑟2=0
        // (−𝑏±√(𝑏2−4𝑎𝑐))/2𝑎 = −ℎ±√(ℎ2−𝑎𝑐)/𝑎

        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b.powf(2.) - a * c;
        if discriminant < 0. {
            // no solutions, ray doesn't hit the sphere
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        let p = ray.at(root);

        rec.t = Some(root);
        rec.p = Some(p);
        let outward_normal = (p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}
