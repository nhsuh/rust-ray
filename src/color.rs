pub mod color {
    use crate::interval::interval::Interval;
    pub use crate::vec3::vec3::Vec3;
    pub use Vec3 as Color;
    #[inline]
    fn linear_to_gamma(linear_component: f64) -> f64 {
        linear_component.sqrt()
    }
    pub fn write_color(pixel_color: &Color, samples_per_pixel: i32) {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();
        let scale = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        const INTENSITY: Interval = Interval::new_params(0.000, 0.999);
        let x_col = (256.0 * INTENSITY.clamp(r)) as i16;
        let y_col = (256.0 * INTENSITY.clamp(g)) as i16;
        let z_col = (256.0 * INTENSITY.clamp(b)) as i16;
        println!("{} {} {}", x_col, y_col, z_col);
    }
}
