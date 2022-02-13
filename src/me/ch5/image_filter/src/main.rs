use image::{GenericImage, GenericImageView, Rgba};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} image", args[0]);
        return;
    }
    let infile = args[1].clone();
    let outfile = format!("{}-out.png", infile);
    println!("{} -> {}", infile, outfile);
    let mut img = image::open(&infile).unwrap();
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let px = img.get_pixel(x, y);
            let r = px[0];
            let g = px[1];
            let b = px[2];
            let a = px[3];
            let neg = Rgba([255 - r, 255 - g, 255 - b, a]);
            img.put_pixel(x, y, neg);
        }
    }
    img.save(&outfile).unwrap();
}
