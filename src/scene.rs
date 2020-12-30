use crate::core::{
    camera::Camera,
    color::Color,
    hit::Hittable,
    hittable_list::HittableList,
    material::{Material, Dielectric, DiffuseMethod, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Length, Point3},
};
use crate::ASPECT_RATIO;
use rand::{random, thread_rng, Rng};
use std::sync::Arc;

pub fn get_camera() -> Camera {
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    // let lookfrom = Point3::new(3., 3., 2.);
    // let lookat = Point3::new(0., 0., -1.);
    let vup = Point3::new(0., 1., 0.);
    let dist_to_focus = 10.; // (lookfrom - lookat).length();
    let aperture = 0.1;// 2.0;
    Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    )
}

pub fn generate_scene(diffuse_method: &mut DiffuseMethod) -> HittableList<impl Hittable> {
    // Materials
    let material_ground = Arc::new(Lambertian::new(
        Color::new_rgb(204, 204, 0),
        *diffuse_method,
    ));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5), *diffuse_method));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    // Objects
    let planet = Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    let sphere_center = Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center));
    let sphere_left = Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    let sphere_left_2 = Arc::new(Sphere::new(Point3::new(-1., 0., -1.), -0.45, material_left));
    let sphere_right = Arc::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    // World
    let mut world = HittableList::new();
    world.add(planet);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_left_2);
    world.add(sphere_right);

    world
}

pub fn random_scene(diffuse_method: &mut DiffuseMethod) -> HittableList<impl Hittable> {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5), *diffuse_method));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    let mut rng = thread_rng();

    // Small spheres
    for au in -11..11 {
        let a = au as f64;
        for bu in -11..11 {
            let b = bu as f64;
            let material_choice = random::<f64>();
            let center = Point3::new(a + 0.9 * random::<f64>(), 0.2, b + 0.9 * random::<f64>());

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = {
                    if material_choice < 0.8 {
                        // Diffuse
                        let albedo = Color::random() * Color::random();
                        Arc::new(Lambertian::new(albedo, *diffuse_method))
                    } else if material_choice < 0.95 {
                        // Metal
                        let albedo = Color::random_limit(0.5, 1.);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        // Glass
                        Arc::new(Dielectric::new(1.5))
                    }
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // Big spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1), *diffuse_method));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.add(Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., material1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2)));
    world.add(Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1., material3)));

    world
}
