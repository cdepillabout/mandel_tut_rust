extern crate cgmath;
extern crate image;
extern crate num;
extern crate num_complex;

use cgmath::BaseNum;
use cgmath::Vector2;
use cgmath::VectorSpace;
use image::ImageBuffer;
use image::ImageLuma8;
use image::Luma;
use num::Complex;
use num::Float;
use num::Num;
use std::fs::File;
use std::path::Path;

struct Bound<T> {
    min: T,
    max: T,
}

impl<T> Bound<T>
where
    T: Num + Copy,
{
    fn scale<U: Into<T>>(&self, img_size: U) -> T {
        (self.max - self.min) / img_size.into()
    }
}

fn main() {
    let max_iterations = 512u16;
    // let img_size = Vec800u16;
    let img_size = Vector2::new(800_u16, 800_u16);

    let cx_bound = Bound {
        min: -2f32,
        max: 1f32,
    };

    let cy_bound = Bound {
        min: -1.5f32,
        max: 1.5f32,
    };

    let c_min : Vector2<f32> = Vector2::new(cx_bound.min, cy_bound.min);
    let c_scale : Vector2<f32> = Vector2::new(cx_bound.scale(img_size.x), cy_bound.scale(img_size.y));

    let mut output_vec = Vec::new();

    // print_buf(foo);

    dbg!(c_min);
    dbg!(c_scale);

    // let img_buf = draw_mandel(max_iterations, &img_size, /* 2.0_f32, */ &c_min, &c_scale, &mut output_vec);
    // print_buf(img_buf);
}

fn simple<S, V, Q>(v: &V, q: &Q) -> ()
where
    S: VectorSpace,
    S::Scalar = f32,
    // V: S<Scalar = f32>,
    // Q: S<Scalar = u16>,
{
}

fn draw_mandel<T, S, V, Q>(
    max_iterations: u16,
    img_size: &V,
    // norm_breakpoint: T,
    c_min: &V,
    c_scale: &V,
    output_vec: &mut Vec<V>,
) -> ()
where
    T: BaseNum + Float + Copy + From<u16>,
    S: VectorSpace,
    V: S<Scalar = T>,
    Q: S<Scalar = u16>,
{
    
    // let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> =
    //     image::ImageBuffer::new(img_size.into(), img_size.into());

    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let x_t: T = (x as u16).into();
    //     let y_t: T = (y as u16).into();
    //     let z_t: T = (y as u16).into();

    //     let cx = cx_bound.min + x_t * cx_bound.scale(img_size);
    //     let cy = cy_bound.min + y_t * cy_bound.scale(img_size);
    //     let cz = cy_bound.min + y_t * cy_bound.scale(img_size);

    //     let c = Complex::new(cx, cy);
    //     let mut z = Complex::new(T::zero(), T::zero());

    //     let mut i = 0;
    //     for t in 0..max_iterations {
    //         if z.norm() > norm_breakpoint {
    //             break;
    //         }
    //         z = z * z + c;
    //         i = t;
    //     }

    //     *pixel = image::Luma([i as u8]);
    // }

    // imgbuf
}

// fn print_buf(imgbuf: ImageBuffer<Luma<u8>, Vec<u8>>) {
//     let path = Path::new("fractal.png");
//     File::create("fractal.png").unwrap();
//     ImageLuma8(imgbuf).save(path).unwrap();
// }
