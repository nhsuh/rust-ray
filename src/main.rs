use rust_ray::camera::camera::Camera;
use rust_ray::color::color::Color;
use rust_ray::hittable_list::hittable_list::HittableList;
use rust_ray::material::material::{Lambertian, Metal, Glass};
use rust_ray::sphere::sphere::Sphere;
fn main() {
    let mut world: HittableList = HittableList::new();

    let material_ground = Box::new(Lambertian::new(Color::new_params(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new_params(0.7, 0.3, 0.3)));
    let material_left = Box::new(Metal::new(Color::new_params(0.8, 0.8, 0.8), 0.5));
    let material_right = Box::new(Glass::new(Color::new_params(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Color::new_params(0.0, 0.0, -1.0),
        0.5,
        material_center
    )));
    world.add(Box::new(Sphere::new(
        Color::new_params(0.0, -100.5, -1.0),
        100.0,
        material_ground
    )));
    world.add(Box::new(Sphere::new(
        Color::new_params(-1.0, 0.0, -1.0),
        0.5,
        material_left
    )));
    world.add(Box::new(Sphere::new(
        Color::new_params(1.0, 0.0, -1.0),
        0.5,
        material_right
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
