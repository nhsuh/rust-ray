pub mod vec3 {
    use crate::rtweekend::rtweekend::*;
    use std::ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
    };
    use std::cmp;
    #[derive(Copy, Clone)]
    pub struct Vec3 {
        array: [f64; 3],
    }
    impl Vec3 {
        pub fn new() -> Vec3 {
            Vec3 {
                array: [0.0, 0.0, 0.0],
            }
        }

        pub fn new_params(x: f64, y: f64, z: f64) -> Vec3 {
            Vec3 { array: [x, y, z] }
        }
        pub fn random() -> Vec3 {
            Vec3 {
                array: [random_float(), random_float(), random_float()],
            }
        }
        pub fn random_range(min: f64, max: f64) -> Vec3 {
            Vec3 {
                array: [
                    random_float_range(min, max),
                    random_float_range(min, max),
                    random_float_range(min, max),
                ],
            }
        }
        pub fn x(&self) -> f64 {
            self.array[0]
        }
        pub fn y(&self) -> f64 {
            self.array[1]
        }
        pub fn z(&self) -> f64 {
            self.array[2]
        }
        pub fn length_squared(&self) -> f64 {
            self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
        }
        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
        pub fn near_zero(&self) -> bool {
            const S: f64 = 1e-8;
            return (self.x().abs() < S) && (self.y().abs() < S) && (self.z().abs() < S)
        }
        #[inline(always)]
        pub fn print_vec(&self) {
            println!("{} {} {}", self.x(), self.y(), self.z())
        }
        pub fn eprint_vec(&self) {
            eprintln!("{} {} {}", self.x(), self.y(), self.z())
        }
        #[inline(always)]
        pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
            (u * v).array.iter().sum()
        }
        #[inline(always)]
        pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
            Vec3 {
                array: [
                    u.array[1] * v.array[2] - u.array[2] * v.array[1],
                    u.array[2] * v.array[0] - u.array[0] * v.array[2],
                    u.array[0] * v.array[1] - u.array[1] * v.array[0],
                ],
            }
        }
        #[inline(always)]
        pub fn unit_vector(&self) -> Vec3 {
            return self / self.length();
        }
        #[inline(always)]
        fn random_in_unit_sphere() -> Vec3 {
            loop {
                let p = Vec3::random_range(-1.0, 1.0);
                if p.length_squared() < 1.0 {
                    return p;
                }
            }
        }
        #[inline(always)]
        pub fn random_unit_vector() -> Vec3 {
            Vec3::random_in_unit_sphere().unit_vector()
        }
        #[inline(always)]
        pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
            let on_unit_sphere: Vec3 = Vec3::random_unit_vector();
            if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
                return on_unit_sphere;
            }
            -on_unit_sphere
        }

        pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
            v -  &(n * 2.0 * Vec3::dot(v, n))
        }
        pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
            let cos_theta = (1.0_f64).min(Vec3::dot(&-(*uv), n));
            let r_out_perp = (uv + &(n * cos_theta)) * etai_over_etat;
            let r_out_parallel = -(*n) * (1.0 - r_out_perp.length_squared()).abs().sqrt();
            r_out_perp + r_out_parallel


        }
    }
    impl Neg for Vec3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vec3 {
                array: [-self.x(), -self.y(), -self.z()],
            }
        }
    }
    impl Index<usize> for Vec3 {
        type Output = f64;

        fn index(&self, idx: usize) -> &Self::Output {
            &self.array[idx]
        }
    }
    impl IndexMut<usize> for Vec3 {
        fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
            &mut self.array[idx]
        }
    }
    impl AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                array: [
                    self.array[0] + rhs.array[0],
                    self.array[1] + rhs.array[1],
                    self.array[2] + rhs.array[2],
                ],
            }
        }
    }
    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Self {
                array: [
                    self.array[0] - rhs.array[0],
                    self.array[1] - rhs.array[1],
                    self.array[2] - rhs.array[2],
                ],
            }
        }
    }
    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = Self {
                array: [
                    self.array[0] * rhs,
                    self.array[1] * rhs,
                    self.array[2] * rhs,
                ],
            }
        }
    }
    impl DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            *self *= 1.0 / rhs
        }
    }
    impl Add for &Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn add(self, rhs: &Vec3) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] + rhs.array[0],
                    self.array[1] + rhs.array[1],
                    self.array[2] + rhs.array[2],
                ],
            }
        }
    }
    impl Add for Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn add(self, rhs: Vec3) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] + rhs.array[0],
                    self.array[1] + rhs.array[1],
                    self.array[2] + rhs.array[2],
                ],
            }
        }
    }
    impl Sub for &Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn sub(self, rhs: &Vec3) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] - rhs.array[0],
                    self.array[1] - rhs.array[1],
                    self.array[2] - rhs.array[2],
                ],
            }
        }
    }
    impl Sub for Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn sub(self, rhs: Vec3) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] - rhs.array[0],
                    self.array[1] - rhs.array[1],
                    self.array[2] - rhs.array[2],
                ],
            }
        }
    }
    impl Mul for &Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn mul(self, rhs: &Vec3) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] * rhs.array[0],
                    self.array[1] * rhs.array[1],
                    self.array[2] * rhs.array[2],
                ],
            }
        }
    }
    impl Mul for Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn mul(self, rhs: Vec3) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] * rhs.array[0],
                    self.array[1] * rhs.array[1],
                    self.array[2] * rhs.array[2],
                ],
            }
        }
    }
    impl Mul<f64> for &Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn mul(self, rhs: f64) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] * rhs,
                    self.array[1] * rhs,
                    self.array[2] * rhs,
                ],
            }
        }
    }
    impl Mul<f64> for Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn mul(self, rhs: f64) -> Vec3 {
            Vec3 {
                array: [
                    self.array[0] * rhs,
                    self.array[1] * rhs,
                    self.array[2] * rhs,
                ],
            }
        }
    }
    impl Div<f64> for &Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn div(self, rhs: f64) -> Vec3 {
            self * (1.0 / rhs)
        }
    }
    impl Div<f64> for Vec3 {
        type Output = Vec3;
        #[inline(always)]
        fn div(self, rhs: f64) -> Vec3 {
            self * (1.0 / rhs)
        }
    }
    pub type Point3 = Vec3;
}
