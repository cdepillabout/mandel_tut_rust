extern crate image;
extern crate num;
extern crate num_complex;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

// use std::fs::File;
// use std::path::Path;
use num::Complex;
use num::Float;
// use num_complex::Complex;
use image::ImageBuffer;
use image::Luma;

struct Bound<T> {
    min: T,
    max: T,
}

// fn calc_scale<T, U>(lala: &Bound<T>, img_size: U) -> T
//     where T: Sub + Div,
//           U: Clone + Into<T>

// {
//   // (self.max - self.min)  / T::from(img_size)
//   // (self.max - self.min)  - T::from(img_size)
//   // T::from(img_size)
//   img_size.into()
// }

impl<T> Bound<T>
where
    T: Sub<Output = T> + Div<Output = T> + Copy,
{
    fn scale<U: Into<T>>(&self, img_size: U) -> T {
        (self.max - self.min) / img_size.into()
    }
}

fn main() {
    let max_iterations = 100u16;
    let img_size = 800u16;

    let cx_bound = Bound {
        min: -2f32,
        max: 1f32,
    };

    let cy_bound = Bound {
        min: -1.5f32,
        max: 1.5f32,
    };

    // print_buf(foo);

    dbg!(cx_bound.scale(img_size));
    dbg!(cy_bound.scale(img_size));

    draw_mandel(max_iterations, img_size, 2.0_f32, &cx_bound, &cy_bound);
}

fn draw_mandel<T>(
    max_iterations: u16,
    img_size: u16,
    norm_breakpoint: T,
    cx_bound: &Bound<T>,
    cy_bound: &Bound<T>,
) -> ImageBuffer<Luma<u8>, Vec<u8>>
where
    T: Float + Copy + From<u16>,
{
    let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.into(), img_size.into());

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x_T: T = (x as u16).into();
        let y_T: T = (y as u16).into();

        let cx = cx_bound.min + x_T * cx_bound.scale(img_size);
        let cy = cy_bound.min + y_T * cy_bound.scale(img_size);

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(T::zero(), T::zero());

        let mut i = 0;
        for t in 0..max_iterations {
            if z.norm() > norm_breakpoint {
                break;
            }
            // z = z * z + c;
            // i = t;
        }
    }

    imgbuf
}
