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
    for _ in 0..1_000_000{
        img.draw_line_bresenham_with_uint(30, 30, 80, 50);
        img.draw_line_bresenham_with_uint(30, 30, 60, 80);
        img.draw_line_bresenham_with_uint(300, 300, 260, 250);
    }
    img.write_to_tga("./test2.tga").unwrap();
}

fn test_float() -> () {
    let mut img = Image::new(WIDTH, HEIGHT);
    for _ in 0..1_000_000{
        img.draw_line_bresenham_with_float(30, 30, 80, 50);
        img.draw_line_bresenham_with_float(30, 30, 60, 80);
        img.draw_line_bresenham_with_float(300, 300, 260, 250);
    }
    img.write_to_tga("./test1.tga").unwrap();
}
