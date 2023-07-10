use std::io::Write;

use ray::Ray;
use world::Camera;

use crate::{vec::Vec3, ray::HitList};

mod ray;
mod math;
mod vec;
mod ppm;
mod world;
mod geom;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    

fn main() -> std::io::Result<()> {

    let mut ppm = ppm::Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let cam = Camera::new(ASPECT_RATIO, 2.0, 1.0);

    let mut world = HitList::new();
    world.0.push(geom::Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    world.0.push(geom::Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));

    for y in (0..IMAGE_HEIGHT).rev() {        
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Scanlines Remaining: {}", y);

        for x in 0..IMAGE_WIDTH {
            let u = (x as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (y as f64) / (IMAGE_HEIGHT - 1) as f64;
            let ray = cam.cast_ray_at(u, v);
            ppm.push(ray.color(world.clone()));
        }
    }

    ppm.read_to_file("output.ppm")?;

    println!("Done!");
    Ok(())
}
