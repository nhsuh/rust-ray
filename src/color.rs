pub mod color {
    pub use crate::vec3::vec3::Vec3;
    pub use Vec3 as Color;
    pub fn write_color(pixel_color: &Color) {
        let x_col = (255.999 * pixel_color.x()) as i16;
        let y_col = (255.999 * pixel_color.y()) as i16;
        let z_col = (255.999 * pixel_color.z()) as i16;
        println!("{} {} {}", x_col, y_col, z_col);
    }
}
