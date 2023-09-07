use rust_ray::color::color::{Color, write_color};
fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_color = Color::new_params((i as f64)/((image_width - 1) as f64), (j as f64)/((image_width - 1) as f64), 0.0);
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone.                 \n");
}
