/*
This code is written based on the code from https://gist.github.com/jonvaldes/607fbc380f816d205afb
which describe how to write TGA file in rust.
*/

use std::ffi::OsStr;
use std::fs::{File, self};
use std::io;
use std::io::Write;
use std::mem;
use std::mem::swap;
use std::path::Path;
use std::slice;

pub struct Image {
    header: Header,
    data: Vec<RGBA>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct RGBA(u8, u8, u8, u8);
#[repr(C, packed)]
#[derive(Default)]
pub struct Header {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    c_map_start: u16,
    c_map_length: u16,
    c_map_depth: u8,
    x_offset: u16,
    y_offset: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,
    image_descriptor: u8,
}

unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(s);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(&s[0]);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}

impl Image {
    pub fn new(width: u16, height: u16) -> Image {
        let data = vec![RGBA(0, 0, 0, 255); (width as u32 * height as u32) as usize];
        let header = Header {
            image_type: 2,
            width,
            height,
            pixel_depth: 32,
            ..Header::default()
        };
        Image { header, data }
    }

    pub fn set_pixel(self: &mut Image, x: u16, y: u16, r: u8, g: u8, b: u8, a: u8) {
        assert!(x < self.header.width);
        assert!(y < self.header.height);
        self.data[(x as i32 + y as i32 * self.header.width as i32) as usize] = RGBA(r, g, b, a);
    }

    pub fn draw_line_bresenham_with_float(self: &mut Image, x0 : u16, y0 : u16, x1 : u16, y1 : u16) {
        let white = RGBA(255, 255, 255, 255);
        let mut swaped = false;
        let (mut xa,mut ya,mut xb,mut yb) = (x0, y0, x1, y1);
        let mut angle = (yb as i32 - ya as i32) as f32 / (xb as i32 - xa as i32) as f32;
        if angle.abs() > 1. {
            swaped = true;
            angle = angle.recip();
            swap(&mut xa, &mut ya);
            swap(&mut xb, &mut yb);
        }
        if xa > xb {
            swap(&mut xa, &mut xb);
            swap(&mut ya, &mut yb);
        }   
        for x in xa..xb {
            let y = (ya as f32 + (x - xa) as f32 * angle) as u16; 
            if swaped {
        // println!("x : {}, y : {}", y, x);
                self.set_pixel(y, x, white.0, white.1, white.2, white.3);
            } else {
        // println!("x : {}, y : {}", x, y);
                self.set_pixel(x, y, white.0, white.1, white.2, white.3);
            }
        }
    }


    pub fn draw_line_bresenham_with_uint(self: &mut Image, x0 : u16, y0 : u16, x1 : u16, y1 : u16) {
        let white = RGBA(255, 255, 255, 255);
        let mut swaped = false;
        let (mut xa,mut ya,mut xb,mut yb) : (i32, i32, i32, i32)= (x0 as i32, y0 as i32, x1 as i32, y1 as i32);
        let mut adder = 1;

        if  (yb - ya).abs() > (xb - xa).abs() {
            swaped = true;
            swap(&mut xa, &mut ya);
            swap(&mut xb, &mut yb);
        }

        if xa > xb {
            swap(&mut xa, &mut xb);
            swap(&mut ya, &mut yb);
        }   

        let (dx , dy ) = (xb - xa, (yb - ya).abs());
        let mut error = 0;
        let mut y = ya as u16;
        if ya > yb {
            adder = -1;
        }
        
        println!("xa : {}, ya : {}", xa, ya);
        for x  in xa as u16..xb as u16 {
            if error >= dx {
                error -= dx;
                y = (y as i32 + adder) as u16; 
            }
            if swaped {
        // println!("x : {}, y : {}", y, x);
                self.set_pixel(y, x, white.0, white.1, white.2, white.3);
            } else {
        // println!("x : {}, y : {}", x, y);
                self.set_pixel(x, y, white.0, white.1, white.2, white.3);
            }
            error += dy;
        }
    }

    pub fn write_to_tga<S:AsRef<OsStr>>(self: &Image, filename: S) -> io::Result<()> {
        match fs::create_dir(Path::new(&filename).parent().unwrap()){
            Err(e) => {
                if e.kind() != std::io::ErrorKind::AlreadyExists {
                    panic!("Can't Create directory : {:?}", e);
                }
            },
            Ok(_) => {},
        }
        let mut f = File::create(filename.as_ref())?;
        unsafe {
            f.write_all(struct_to_u8_slice(&self.header))?;
            f.write_all(slice_to_u8_slice(&self.data[..]))?;
        }
        Ok(())
    }

}

#[cfg(test)]
mod compare_float_and_uint{
    use super::*;
    const TEST_PATH : &str = "./test/";
    const WIDTH : u16 = 640;
    const HEIGHT : u16 = 480;

    fn cmp(img1 : Vec<RGBA>, img2 : Vec<RGBA>) -> Result<(), String> {
        if img1.len() != img2.len() {
            return Err(format!("Vector length is not equal"))
        }
        for y in 0..HEIGHT
        {
            for x in 0..WIDTH {
                let pexel_1 = &img1[(x as u32 + y as u32 * WIDTH as u32) as usize];
                let pexel_2 = &img2[(x as u32+ y as u32 * WIDTH as u32) as usize];
                if pexel_1 != pexel_2 {
                    return Err(format!("({}, {}) pexel_1 : {:?}, pexel_2 : {:?}", x, y, pexel_1, pexel_2));
                }
            }
        }
        Ok(())
    }

    #[test]
    fn direction_1() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(10, 10, 300, 200);
        img2.draw_line_bresenham_with_uint(10, 10, 300, 200);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test1-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test1-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_2() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(10, 10, 100,400);
        img2.draw_line_bresenham_with_uint(10, 10, 100, 400);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test2-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test2-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_3() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(200, 100, 100, 400);
        img2.draw_line_bresenham_with_uint(200, 100, 100, 400);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test3-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test3-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_4() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(500,100,100,200);
        img2.draw_line_bresenham_with_uint(500,100,100,200);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test4-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test4-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_5() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(300, 300, 100, 200);
        img2.draw_line_bresenham_with_uint(300, 300, 100, 200);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test5-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test5-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_6() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(300, 300, 200, 100);
        img2.draw_line_bresenham_with_uint(300, 300, 200, 100);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test6-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test6-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_7() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(100, 500, 200, 100);
        img2.draw_line_bresenham_with_uint(100, 500, 200, 100);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test7-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test7-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
    #[test]
    fn direction_8() -> Result<(), String> {
        let mut img1 = Image::new(WIDTH, HEIGHT);
        let mut img2 = Image::new(WIDTH, HEIGHT);
        img1.draw_line_bresenham_with_float(100, 200, 500, 100);
        img2.draw_line_bresenham_with_uint(100, 200, 500, 100);
        img1.write_to_tga(TEST_PATH.to_owned()+ "test8-1.tga").unwrap();
        img2.write_to_tga(TEST_PATH.to_owned()+ "test8-2.tga").unwrap();
        cmp(img1.data, img2.data).unwrap();
        Ok(())
    }
}
