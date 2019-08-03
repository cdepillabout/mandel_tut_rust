extern crate image;
extern crate num_complex;
 
use std::fs::File;
use std::path::Path;
use num_complex::Complex;
 
fn main(){
    let max_iterations = 512u16;
    let img_size = 800u32;
    let cxmin = -2f32;
    let cymin = -1.5f32;
    let cxmax = 1f32;
    let cymax = 1.5f32;
    let scalex = calc_scalex(cxmax, cxmin, img_size);
    let scaley = calc_scaley(cymax, cymin, img_size);
  
    draw_mandel(max_iterations, img_size, cxmin, cymin, scalex, scaley);
}

fn calc_scalex(cxmax:f32, cxmin:f32, img_size:u32) -> f32{
    (cxmax - cxmin) / img_size as f32
}

fn calc_scaley(cymax:f32, cymin:f32, img_size:u32) -> f32{
    (cymax - cymin) / img_size as f32
}

fn draw_mandel(max_iterations:u16, img_size:u32, cxmin:f32, cymin:f32, scalex:f32, scaley:f32) {

    let mut imgbuf = image::ImageBuffer::new(img_size, img_size);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;
 
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);
 
        let mut i = 0;
        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }
 
        *pixel = image::Luma([i as u8]);
    }

    let path = Path::new("fractal.png");
    let fout = &mut File::create("fractal.png").unwrap();
    image::ImageLuma8(imgbuf).save(path).unwrap();

}