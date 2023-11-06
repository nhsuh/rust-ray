pub mod rtweekend {
    use rand::{thread_rng, Rng};
    pub const INF: f64 = f64::INFINITY;
    pub const PI: f64 = 3.1415926535897932385;

    #[inline]
    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }
    #[inline]
    pub fn random_float() -> f64 {
        thread_rng().gen_range(0.0..1.0)
    }
    #[inline]
    pub fn random_float_range(min: f64, max: f64) -> f64 {
        thread_rng().gen_range(min..max)
    }
}
