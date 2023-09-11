pub mod sphere {
    pub use crate::hittable::hittable::{Hittable, Point3, Vec3};
    use crate::interval::interval::Interval;
    pub struct Sphere {
        center: Point3,
        radius: f64,
    }
    impl Sphere {
        pub fn new(center: Point3, radius: f64) -> Sphere {
            Sphere { center, radius }
        }
    }
    impl Hittable for Sphere {
        fn hit(
            &self,
            r: &crate::hittable::hittable::Ray,
            ray_t: Interval,
            rec: &mut crate::hittable::hittable::HitRecord,
        ) -> bool {
            let oc: Vec3 = &r.orig - &self.center;
            let a = r.dir.length_squared();
            let half_b = Vec3::dot(&oc, &(r.dir));
            let c = oc.length_squared() - (&self.radius * &self.radius);
            let discriminant = half_b * half_b - (a * c);
            if discriminant < 0.0 {
                return false;
            }
            let sqrtd = discriminant.sqrt();

            let root = (-half_b - sqrtd) / a;
            if !ray_t.surrounds(root) {
                let root = (-half_b + sqrtd) / a;
                if !ray_t.surrounds(root) {
                    return false;
                }
            }

            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);

            return true;
        }
    }
}
