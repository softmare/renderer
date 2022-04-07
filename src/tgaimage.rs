/*
This code is written based on the code from https://gist.github.com/jonvaldes/607fbc380f816d205afb
which describe how to write TGA file in rust.
*/

use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::slice;

pub struct Image {
    header: Header,
    data: Vec<RGBA>,
}
#[derive(Clone)]
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
        let data = vec![RGBA(0, 0, 0, 0); (width as u32 * height as u32) as usize];
        let header = Header {
            image_type: 2,
            width,
            height,
            pixel_depth: 32,
            ..Header::default()
        };
        Image { header, data }
    }

    pub fn apply_gamma(self: &mut Image, gamma: f32) {
        for c in self.data.iter_mut() {
            let RGBA(r, g, b, _a) = *c;
            let fr = ((r as f32) / 255.0).powf(gamma);
            let fg = ((g as f32) / 255.0).powf(gamma);
            let fb = ((b as f32) / 255.0).powf(gamma);
            c.0 = (fr * 255.0) as u8;
            c.1 = (fg * 255.0) as u8;
            c.2 = (fb * 255.0) as u8;
        }
    }

    pub fn set_pixel(self: &mut Image, x: u16, y: u16, r: u8, g: u8, b: u8, a: u8) {
        assert!(x < self.header.width);
        assert!(y < self.header.height);
        assert!(x >= 0);
        assert!(y >= 0);
        self.data[(x as i32 + y as i32 * self.header.width as i32) as usize] = RGBA(r, g, b, a);
    }

    pub fn write_to_tga(self: &Image, filename: &str) -> io::Result<()> {
        let mut f = File::create(filename)?;
        unsafe {
            f.write_all(struct_to_u8_slice(&self.header))?;
            f.write_all(slice_to_u8_slice(&self.data[..]))?;
        }
        Ok(())
    }
}
