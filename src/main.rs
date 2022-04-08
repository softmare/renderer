mod tgaimage;
use crate::tgaimage::Image;

const WIDTH: u16 = 640;
const HEIGHT: u16 = 480;

fn main() {
    test_float();
    test_int();
}
fn test_int() -> () {
    let mut img = Image::new(WIDTH, HEIGHT);
    img.draw_line_bresenham_with_uint(30, 30, 100, 50);
    img.draw_line_bresenham_with_uint(30, 30, 200, 300);
    img.draw_line_bresenham_with_uint(300, 300, 200, 100);
    img.draw_line_bresenham_with_uint(500, 300, 100, 10);
    img.write_to_tga("./test2.tga").unwrap();
}

fn test_float() -> () {
    let mut img = Image::new(WIDTH, HEIGHT);
    img.draw_line_bresenham_with_float(30, 30, 100, 50);
    img.draw_line_bresenham_with_float(30, 30, 200, 300);
    img.draw_line_bresenham_with_float(300, 300, 200, 100);
    img.draw_line_bresenham_with_float(500, 300, 100, 10);
    img.write_to_tga("./test1.tga").unwrap();
}
