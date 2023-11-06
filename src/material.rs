pub mod material {
    use dyn_clone::DynClone;
    use crate::color::color::Color;
    use crate::vec3::vec3::Vec3;
    use crate::hittable::hittable::{HitRecord, Ray};
    pub trait Material: DynClone {
        fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)
            -> bool;
    }
    dyn_clone::clone_trait_object!(Material);
    #[derive(Clone)]
    pub struct Lambertian {
        pub albedo: Color,
    }
    impl Lambertian {
        pub fn new(a: Color) -> Lambertian {
            Lambertian { albedo: a }
        }
    }
    impl Material for Lambertian {
        fn scatter(
            &self,
            _r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
            let mut  scatter_direction = rec.normal + Vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }
            *scattered = Ray::new_params(&rec.p, &scatter_direction);
            *attenuation = self.albedo;
            true

        }
    }
    #[derive(Clone)]
    pub struct Metal {
        pub albedo: Color, 
        pub fuzz: f64
    }
    impl Metal {
        pub fn new(a: Color, f: f64) -> Metal {
            if f < 1.0 {
                return Metal { albedo: a, fuzz: f}
            }
            Metal { albedo: a, fuzz: 1.0}
        }
    }
    impl Material for Metal {
        fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
            let reflected: Vec3 = Vec3::reflect(&(r_in.dir), &(rec.normal));
            *scattered = Ray::new_params(&rec.p, &(reflected +  Vec3::random_unit_vector() * self.fuzz ));
            *attenuation = self.albedo;
            Vec3::dot(&(scattered.dir), &(rec.normal)) > 0.0

        }
    }
    #[derive(Clone)]
    pub struct Glass {
        pub albedo: Color,
    }
    impl Glass {   
        pub fn new(a: Color) -> Glass {
            Glass { albedo: a }
        }

    }
    impl Material for Glass {
        fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
            *attenuation = Color::new_params(1.0, 1.0, 1.0);
            *scattered = Ray::new_params(&rec.p, &((r_in.dir) + Color::new_params(0.5, 0.5, 0.2)));
            true

        }
    }
}
