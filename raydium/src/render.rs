use std::{rc::Rc, cell::{RefCell, Ref, RefMut}, borrow::BorrowMut};

use image::{RgbImage, DynamicImage, ImageBuffer, Rgb};
use rand::Rng;
use sdl2::{Sdl, VideoSubsystem, video::Window, render::{Canvas, Texture, TextureCreator}, pixels::{Color, PixelFormatEnum}, sys::SDL_Texture, event::Event, keyboard::Keycode, rect::Rect};

use crate::{world::{Camera}, math::{RectSize, IOResult, clamp}, ray::{Hittable, HitList}, geom::SdlTexture, vec::Vec3};

pub struct RadWindow {
    ctx: Sdl,
    video: VideoSubsystem,
    canvas: RefCell<Canvas<sdl2::video::Window>>,
    event_pump: RefCell<sdl2::EventPump>
}

pub struct Renderer {
    window: RadWindow, 
    camera: Camera,
    texture_creator: Rc<TextureCreator<sdl2::video::WindowContext>>
}

impl RadWindow {
    pub fn build_new(width: u32, height: u32, title: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let win = video
            .window(title, width, height)
            .vulkan()
            .resizable()
            .build()?;
        
        let mut canvas = win.into_canvas().build()?;
    
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        Ok((RadWindow {
            ctx: sdl.clone(),
            video,
            canvas: RefCell::new(canvas),
            event_pump: RefCell::new(sdl.event_pump()?)
        }))
    }

    pub fn events(&self) -> Ref<sdl2::EventPump> { self.event_pump.borrow() }
    pub fn events_mut(&self) -> RefMut<sdl2::EventPump> { self.event_pump.borrow_mut()}

}

impl Renderer {

    const DEFAULT_NUM_SAMPLES: u32 = 100;

    pub fn new(window: RadWindow, camera: Camera) -> Self {

        let c = {
            let c = window.canvas.borrow();
            let tc = Rc::new(c.texture_creator());
            tc
        };
        
        Self { 
            window,
            camera, 
            texture_creator: c
        }
    }

    pub const fn radwindow(&self) -> &RadWindow { &self.window }
    pub fn canvas(&self) -> Ref<Canvas<sdl2::video::Window>> { self.window.canvas.borrow() }
    pub fn canvas_mut(&self) -> RefMut<Canvas<sdl2::video::Window>> { self.window.canvas.borrow_mut() }    pub const fn window(&self) -> &RadWindow { &self.window }

    pub fn create_static_texture(&self, width: u32, height: u32) -> IOResult<SdlTexture> {
        let t = self.texture_creator
            .create_texture_static(PixelFormatEnum::RGB24, width, height)?;
        let st = SdlTexture::new(t, Rect::new(0, 0, width, height));
        Ok(st)
    }

    pub fn create_streaming_texture(&self, width: u32, height: u32) -> IOResult<SdlTexture> {
        let t = self.texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)?;
        let st = SdlTexture::new(t, Rect::new(0, 0, width, height));
        Ok(st)
    }

    pub fn clear_black(&self) {
        let mut c = self.canvas_mut();
        c.set_draw_color(Color::BLACK);
        c.clear();
    }

    pub fn draw_texture(&self, tex: &Texture, tex_rect: &Rect) {
        let mut c = self.canvas_mut();
        let _ = c.copy(tex, None, Some(*tex_rect));
    }

    pub fn swap_buffers(&self) {
        let mut c = self.canvas_mut();
        c.present();
    }

    // TODO :: This shit slow. Make it concurrent/multithreaded

    pub fn render_world_to_image<T: Hittable>(&self, world: &HitList<T>, size: RectSize) -> IOResult<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let RectSize {width, height} = size;

        let mut image = DynamicImage::new_rgb8(width, height);
        let buffer = image.as_mut_rgb8().expect("Failed to convert image to rgb8");

        let mut rng = rand::thread_rng();
    
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Scanlines Remaining: {}\n{}%", height-1 - y,  y as f64 / (height - 1) as f64 * 100.0);
            // let u = (x as f64) / (width - 1) as f64;
            // let v = (y as f64) / (height - 1) as f64;
            let color = {
                let mut color = Vec3::zero();

                for n in 0..Self::DEFAULT_NUM_SAMPLES - 1 {
                    let u = (x as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / (height - 1) as f64;

                    let ray = self.camera.cast_ray(u, v);
                    color = color + ray.color(world.clone());
                }
                color
            };



            write_color(pixel, &color, Self::DEFAULT_NUM_SAMPLES)
            // pixel[0] = (ray.color(world.clone()).x() * 255.999) as u8;
            // pixel[1] = (ray.color(world.clone()).y() * 255.999) as u8;
            // pixel[2] = (ray.color(world.clone()).z() * 255.999) as u8;
        }
    
        if let Some(image) = image.flipv().as_rgb8() { 
            Ok(image.to_owned()) 
        } else { 
            Err("Failed to convert image to rgb8".into()) 
        }
        
    }
    
}

fn write_color(pixel: &mut Rgb<u8>, color: &Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = color.x() * scale;
    let g = color.y() * scale;
    let b = color.z() * scale;
    pixel[0] = (255.0 * clamp(r, 0.0, 0.999)) as u8;
    pixel[1] = (255.0 * clamp(g, 0.0, 0.999)) as u8;
    pixel[2] = (255.0 * clamp(b, 0.0, 0.999)) as u8;
}

pub trait Drawable {
    fn draw(&self, renderer: &Renderer);
}
