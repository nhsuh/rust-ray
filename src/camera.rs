pub mod camera {
    use crate::color::color::*;
    use crate::hittable::hittable::*;
    use crate::interval::interval::Interval;
    use crate::ray::ray::Ray;
    use crate::rtweekend::rtweekend::*;
    use crate::vec3::vec3::Vec3;
    pub struct Camera {
        pub aspect_ratio: f64,
        pub image_width: i32,
        pub samples_per_pixel: i32,
        pub max_depth: i32,
        image_height: i32,
        center: Point3,
        pixel00_loc: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
    }
    impl Camera {
        pub fn render(&mut self, world: &dyn Hittable) {
            self.initialize();
            println!("P3\n{} {}\n255", self.image_width, self.image_height);

            for j in 0..self.image_height {
                eprintln!("\rScanlines remaining: {} ", (self.image_height - j));
                for i in 0..self.image_width {
                    let mut pixel_color = Color::new();
                    for _sample in 0..self.samples_per_pixel {
                        let r: Ray = self.get_ray(i, j);
                        pixel_color += Camera::ray_color(&r, self.max_depth, world);
                    }
                    write_color(&pixel_color, self.samples_per_pixel);
                }
            }
            eprintln!("\rDone.                 \n");
        }
        fn initialize(&mut self) {
            self.image_height = if ((self.image_width as f64 / self.aspect_ratio) as i32) < 1 {
                1
            } else {
                (self.image_width as f64 / self.aspect_ratio) as i32
            };

            let focal_length = 1.0;
            let viewport_height = 2.0;
            let viewport_width =
                viewport_height * (self.image_width as f64 / self.image_height as f64);
            let camera_center = Point3::new();

            let viewport_u = Vec3::new_params(viewport_width, 0.0, 0.0);
            let viewport_v = Vec3::new_params(0.0, -viewport_height, 0.0);
            self.pixel_delta_u = &viewport_u / self.image_width as f64;
            self.pixel_delta_v = &viewport_v / self.image_height as f64;

            let viewport_upper_left = &camera_center
                - &Point3::new_params(0.0, 0.0, focal_length)
                - &viewport_u / 2.0
                - &viewport_v / 2.0;
            self.pixel00_loc =
                viewport_upper_left + ((&self.pixel_delta_u + &self.pixel_delta_v) * 0.5);
        }
        fn get_ray(&self, i: i32, j: i32) -> Ray {
            let pixel_center = self.pixel00_loc
                + (self.pixel_delta_u * i as f64)
                + (self.pixel_delta_v * j as f64);
            let pixel_sample = pixel_center + self.pixel_sample_square();

            let ray_origin = self.center;
            let ray_direction = pixel_sample - ray_origin;

            Ray {
                orig: ray_origin,
                dir: ray_direction,
            }
        }
        fn pixel_sample_square(&self) -> Vec3 {
            let px = -0.5 * random_float();
            let py = -0.5 * random_float();
            (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
        }
        fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
            let mut rec = HitRecord::new();
            if depth <= 0 {
                return Color::new();
            }
            if world.hit(r, Interval::new_params(0.001, INF), &mut rec) {
                let mut scattered: Ray = Ray::new();
                let mut attenuation: Color = Color::new();
                if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * Camera::ray_color(&scattered, depth - 1, world)
                }
                return Color::new();
            }
            let unit_direction = Vec3::unit_vector(&r.dir);
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::new_params(1.0, 1.0, 1.0) * (1.0 - a) + Color::new_params(0.5, 0.7, 1.0) * a
        }
    }
    impl Default for Camera {
        fn default() -> Self {
            Self {
                aspect_ratio: 1.0,
                samples_per_pixel: 1,
                max_depth: 10,
                image_width: 100,
                image_height: 100,
                center: Point3::new(),
                pixel00_loc: Point3::new(),
                pixel_delta_u: Vec3::new(),
                pixel_delta_v: Vec3::new(),
            }
        }
    }
}
