use crate::core::{
    vec3::{Color, Point3, Vec3},
    hit::Hittable,
};

const BLACK: Color = Color::new(0., 0., 0.);

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
    pub fn color(&self, world: &dyn Hittable, ray_bounce: usize) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        // Recursion guard for near objects (cracks)
        if ray_bounce == 0 {
            return BLACK;
        }

        if let Some(record) = world.hit(self, 0.001, f64::INFINITY) {
            if let Some(scatter) = record.material.scatter(self, &record) {
                return scatter.attenuation * scatter.ray.color(world, ray_bounce - 1);
            }
            return BLACK;
        }
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.);
        (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
    }

}
