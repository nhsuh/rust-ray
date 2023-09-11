pub mod interval {
    use crate::rtweekend::rtweekend::INF;
    pub struct Interval {
        pub min: f64, 
        pub max: f64
    }
    impl Interval {
        pub fn  new() -> Interval {
            Interval { min: -INF, max: INF }
        }
        pub const fn new_params(min: f64, max: f64) -> Interval {
            Interval { min, max }
        }
        pub fn contains(&self, x: f64) -> bool {
            self.min <= x && self.max >= x
        }
        pub fn surrounds(&self, x: f64) -> bool {
            self.min < x && self.max > x
        }
    }
    const EMPTY: Interval = Interval::new_params(INF, -INF);
    const UNIVERSE: Interval = Interval::new_params(-INF, INF);
}