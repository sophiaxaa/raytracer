fn main() {

    let image_width: i32 = 256;
    let image_height: i32 = 356;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height{
            for i in 0..image_width{
                let r = i as f64 / (image_width - 1) as f64;
                let g = j as f64 / (image_height - 1) as f64;
                let b = 0.0;

                let ir = (255.999 * r) as i32;
                let ig = (255.999 * g) as i32;
                let ib = (255.999 * b) as i32;

                println!("{} {} {}", ir, ig, ib);

            }
    }


}