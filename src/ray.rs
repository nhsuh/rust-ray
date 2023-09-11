pub mod ray {
    pub use crate::vec3::vec3::{Point3, Vec3};
    pub struct Ray {
        pub orig: Point3,
        pub dir: Vec3,
    }

    impl Ray {
        pub fn new() -> Ray {
            Ray {
                orig: Point3::new(),
                dir: Vec3::new(),
            }
        }
        pub fn new_params(orig: &Point3, dir: &Vec3) -> Ray {
            Ray {
                orig: *orig,
                dir: *dir,
            }
        }
        pub fn at(&self, t: f64) -> Point3 {
            &self.orig + &(&self.dir * t)
        }
    }
}
