mod tgaimage;
use crate::tgaimage::Image;

const WIDTH: u16 = 640;
const HEIGHT: u16 = 480;

fn main() {
    test_float();
}

fn test_float() -> () {
    let mut img = Image::new(WIDTH, HEIGHT);
    img.draw_line_bresenham_with_float(30, 30, 100, 50);
    img.draw_line_bresenham_with_float(30, 30, 200, 300);
    img.draw_line_bresenham_with_float(300, 300, 200, 100);
    img.draw_line_bresenham_with_float(500, 300, 100, 10);
    img.write_to_tga("./target/debug/test1.tga").unwrap();
}

fn create_test_image() -> () {
    let mut img = Image::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = ((x ^ y) % 256) as u8;
            let g = ((x + y) % 256) as u8;
            let b = ((y.wrapping_sub(x)) % 256) as u8;
            img.set_pixel(x as u16, y as u16, r, g, b, 124);
        }
    }
    img.apply_gamma(2.2);
    img.write_to_tga("./target/debug/test.tga").unwrap();
}
