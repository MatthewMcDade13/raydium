use std::io::Write;

use math::ftou8;
use ray::Ray;
use world::Camera;

use crate::{vec::Vec3, ray::HitList};
use crate::geom::Sphere;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


mod ray;
mod math;
mod vec;
mod ppm;
mod world;
mod geom;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    

fn main() -> Result<(), Error> {

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(IMAGE_HEIGHT as f64, IMAGE_WIDTH as f64);
        WindowBuilder::new()
            .with_title("Raydium")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // let cam = Camera::new(ASPECT_RATIO, 2.0, 1.0);

    // let mut world = HitList::new();
    // world.0.push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    // world.0.push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    // let (mut frame, mut pixels) = {
    //     let PhysicalSize { width, height } = window.inner_size(); 
    //     let f: Vec<Vec3> = Vec::with_capacity((width * height) as usize);
    //     let mut ps= {
    //         let texture = SurfaceTexture::new(width, height, &window);
    //         Pixels::new(width, height, texture)?
    //     };
    //     (f, ps)
    // };

    
    
    event_loop.run(move |event, _, control_flow| {
    //     if let Event::RedrawRequested(_) = event {
    //         // let PhysicalSize { width, height } = window.inner_size();

    //         // let frame = pixels.frame_mut();
    //         // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
    //         //     let u = i % width as usize;
    //         //     let v = i / width as usize;

    //         //     let ray = cam.cast_ray_at(u as f64, v as f64);
                
    //         //     let Vec3(r, g, b) = ray.color(world.clone());
    //         //     pixel.copy_from_slice(&[
    //         //         ftou8(r),
    //         //         ftou8(g),
    //         //         ftou8(b),
    //         //         255
    //         //     ]);

    //         // }

    //         // if let Err(e) = pixels.render() {
    //         //     error!("pixels.render() failed: {}", e);
    //         //     *control_flow = ControlFlow::Exit;
    //         //     return();
    //         // }
    //     }


    //     if input.update(&event) {
    //         // Close events
    //         if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
    //             *control_flow = ControlFlow::Exit;
    //             return();
    //         }

    //         window.request_redraw();
    //     }
    // })
    // let mut ppm = ppm::Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let cam = Camera::new(ASPECT_RATIO, 2.0, 1.0);

    // let mut world = HitList::new();
    // world.0.push(geom::Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    // world.0.push(geom::Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));

    // for y in (0..IMAGE_HEIGHT).rev() {        
    //     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //     println!("Scanlines Remaining: {}", y);

    //     for x in 0..IMAGE_WIDTH {
    //         let u = (x as f64) / (IMAGE_WIDTH - 1) as f64;
    //         let v = (y as f64) / (IMAGE_HEIGHT - 1) as f64;
    //         let ray = cam.cast_ray_at(u, v);
    //         ppm.push(ray.color(world.clone()));
    //     }
    // }

    // println!("Done!");
    // Ok(())
}

