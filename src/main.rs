use rust_ray::hittable::hittable::{HitRecord, Hittable};
use rust_ray::hittable_list::hittable_list::HittableList;
use rust_ray::rtweekend::rtweekend::INF;
use rust_ray::interval::interval::Interval;


use rust_ray::color::color::{write_color, Color};
use rust_ray::ray::ray::Ray;
use rust_ray::sphere::sphere::Sphere;
use rust_ray::vec3::vec3::Point3;
use rust_ray::vec3::vec3::Vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, Interval::new_params(0.0, INF), &mut rec) {
        return (rec.normal + Color::new_params(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(&r.dir);
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new_params(1.0, 1.0, 1.0) * (1.0 - a) + Color::new_params(0.5, 0.7, 1.0) * a
}
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;

    let image_height = if ((image_width as f64 / aspect_ratio) as i32) < 1 {
        1
    } else {
        (image_width as f64 / aspect_ratio) as i32
    };

    let mut world: HittableList = HittableList::new();

    world.add(Box::new(Sphere::new(
        Color::new_params(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(Sphere::new(
        Color::new_params(0.0, -100.5, -1.0),
        100.0,
    )));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new();

    let viewport_u = Vec3::new_params(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new_params(0.0, -viewport_height, 0.0);
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    let viewport_upper_left = &camera_center
        - &Point3::new_params(0.0, 0.0, focal_length)
        - &viewport_u / 2.0
        - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + ((&pixel_delta_u + &pixel_delta_v) * 0.5);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &camera_center;
            let r = Ray::new_params(&camera_center, &ray_direction);
            let pixel_color = ray_color(&r, &world);
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone.                 \n");
}
