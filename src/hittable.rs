pub mod hittable {
    use crate::color::color::Color;
    use crate::interval::interval::Interval;
    use crate::material::material::{Lambertian, Material};
    pub use crate::ray::ray::Ray;
    pub use crate::vec3::vec3::{Point3, Vec3};
    #[derive(Clone)]
    pub struct HitRecord {
        pub p: Point3,
        pub normal: Vec3,
        pub mat: Box<dyn Material>,
        pub t: f64,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn new() -> HitRecord {
            HitRecord {
                p: Point3::new(),
                normal: Vec3::new(),
                mat: Box::new(Lambertian {
                    albedo: Color::new(),
                }),
                t: 0.0,
                front_face: false,
            }
        }
        pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
            self.front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
            self.normal = if self.front_face {
                *outward_normal
            } else {
                -*outward_normal
            };
        }
    }
    pub trait Hittable {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    }
}
