extern crate sdl2;

use std::io::Write;
use image::{ImageBuffer, DynamicImage};
use math::ftou8;
use ray::Ray;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use world::Camera;

use crate::{vec::Vec3, ray::HitList};
use crate::geom::Sphere;
use log::error;

mod ray;
mod math;
mod vec;
mod ppm;
mod world;
mod geom;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let sdl = sdl2::init().expect("Failed to initialize SDL2");
    let video = sdl.video().expect("Failed to initialize SDL2 video subsystem");
    let win = video
        .window("Raydium", IMAGE_WIDTH, IMAGE_HEIGHT)
        .vulkan()
        .resizable()
        .build()?;
    
    let mut canvas = win.into_canvas().build()?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    /**********************************************
     * 
     */
    let cam = Camera::new(ASPECT_RATIO, 2.0, 1.0);

    let mut world = HitList::new();
    world.0.push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.0.push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let mut image = DynamicImage::new_rgb8(IMAGE_WIDTH, IMAGE_HEIGHT);
    let buffer = image.as_mut_rgb8().expect("Failed to convert image to rgba8");

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u = (x as f64) / (IMAGE_WIDTH - 1) as f64;
        let v = (y as f64) / (IMAGE_HEIGHT - 1) as f64;
        let ray = cam.cast_ray_at(u, v);
        pixel[0] = (ray.color(world.clone()).x() * 255.999) as u8;
        pixel[1] = (ray.color(world.clone()).y() * 255.999) as u8;
        pixel[2] = (ray.color(world.clone()).z() * 255.999) as u8;
    }
    image = image.flipv();
    image
        .save("_image.ppm")
        .expect("Failed to save image");


    /***********************************/

    let tex_creator = canvas.texture_creator();
    let mut tex = tex_creator.create_texture_static(sdl2::pixels::PixelFormatEnum::RGB24, IMAGE_WIDTH, IMAGE_HEIGHT)?;
    let _ = tex.update(None, image.as_rgb8().expect("Failed to convert image to rgba8"), (IMAGE_WIDTH * 3) as usize)?;

    let mut events = sdl.event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }


        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // canvas.draw_rect(Rect::new(0, 0, IMAGE_WIDTH, IMAGE_HEIGHT));
        canvas.copy(&tex, None, Some(Rect::new(0, 0, IMAGE_WIDTH, IMAGE_HEIGHT)))?;
        canvas.present();
    };
    Ok(())
}

