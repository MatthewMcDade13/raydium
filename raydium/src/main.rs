use std::io::Write;

use world::Camera;

use crate::vec::Vec3;

mod ray;
mod math;
mod vec;
mod ppm;
mod world;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
    

fn main() -> std::io::Result<()> {

    let mut ppm = ppm::Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let cam = new_camera(ASPECT_RATIO, 2.0);

    let cam_left_corner = {
        cam.origin - 
        cam.horizontal.div_scalar(2.0) -
        cam.vertical.div_scalar(2.0) - 
        Vec3(0.0, 0.0, cam.focal_length)
    };

    for y in (0..IMAGE_HEIGHT).rev() {        
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Scanlines Remaining: {}", y);

        for x in 0..IMAGE_WIDTH {
            let r = (x as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (y as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = 255.99 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;
            ppm.push(vec::Vec3(ir, ig, ib));
        }
    }

    ppm.read_to_file("output.ppm")?;

    println!("Done!");
    Ok(())
}

fn new_camera(aspect_ratio: f64, viewport_h: f64) -> Camera {
    let viewport_w = aspect_ratio * viewport_h;
    Camera {
        viewport_h,
        viewport_w,
        focal_length: 1.0,
        origin: vec::Vec3(0.0, 0.0, 0.0),
        horizontal: vec::Vec3(viewport_w, 0.0, 0.0),
        vertical: vec::Vec3(0.0, viewport_h, 0.0),
    }
}