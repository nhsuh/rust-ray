use std::io::{self, Write};
fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            eprintln!("\rScanlines remaining: {} ", (image_height - j));
            io::stdout().flush().unwrap();
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0 as f64;

            let ir  = (255.999 * r) as i8;
            let ig = (255.999 * g) as i8;
            let ib = (255.999 * b) as i8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!( "\rDone.                 \n");
}