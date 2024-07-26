mod vec3;
use std::io::{self, Write};

use vec3::{Color, Point3, Vec3, write_color};


fn main() {

    let image_width = 256;
    let image_height = 256;

        //open output 
        let stdout = io:: stdout();
        let mut handle = stdout.lock();

        writeln!(handle, "P3\n{} {}\n255", image_width, image_height).unwrap();


          for j in (0..image_height).rev(){

        //for loading information
        eprint!("\rScanlines remaining: {}", j);

            for i in 0..image_width{
                let pixel_color = Color::new(
                    i as f64 / (image_width - 1) as f64,
                    j as f64 / (image_height - 1) as f64,
                    0.0,
                );

        
                write_color(&mut handle, &pixel_color);

            }
    }
    eprintln!("\rDone!         ");
 
}