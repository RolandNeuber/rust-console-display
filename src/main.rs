use display::Pixel;

fn main() {
    let pix1: Pixel = Pixel::quad_pixel(
        true, false, 
        false, true 
    );

    println!("{}", pix1.get_char());
}
