pub mod hittable_list {
    use crate::hittable::hittable::{HitRecord, Hittable, Ray};
    use crate::interval::interval::Interval;
    pub struct HittableList {
        objects: Vec<Box<dyn Hittable>>,
    }

    impl HittableList {
        pub fn new() -> HittableList {
            HittableList {
                objects: Vec::new(),
            }
        }
        pub fn new_params(object: Box<dyn Hittable>) -> HittableList {
            HittableList {
                objects: vec![object],
            }
        }
        pub fn clear(&mut self) {
            self.objects.clear()
        }
        pub fn add(&mut self, object: Box<dyn Hittable>) {
            self.objects.push(object)
        }
    }
    impl Hittable for HittableList {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
            let mut temp_rec: HitRecord = HitRecord::new();
            let mut hit_anything: bool = false;
            let mut closest_so_far = ray_t.max;

            for object in &self.objects {
                if object.hit(r, Interval::new_params(ray_t.min, closest_so_far), &mut temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    *rec = temp_rec;
                }
            }
            hit_anything
        }
    }
}
