pub mod rtweekend {
    pub const INF: f64 = f64::INFINITY;
    pub const PI: f64 = 3.1415926535897932385;

    #[inline]
    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }
}
