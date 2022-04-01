mod tgaimage;
use crate::tgaimage::Image;

fn main() {
    create_test_image();
}

fn create_test_image() -> () {
    const WIDTH: u16 = 640;
    const HEIGHT: u16 = 480;

    let mut img = Image::new(WIDTH, HEIGHT);

    for y in 0u32..480 {
        for x in 0u32..640 {
            let r = ((x ^ y) % 256) as u8;
            let g = ((x + y) % 256) as u8;
            let b = ((y.wrapping_sub(x)) % 256) as u8;
            img.set_pixel(x as i32, y as i32, r, g, b, 124);
        }
    }
    img.apply_gamma(2.2);
    img.write_to_tga("./target/debug/test.tga").unwrap();
}
