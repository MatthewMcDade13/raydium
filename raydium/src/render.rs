use std::{rc::Rc, cell::{RefCell, Ref, RefMut}, borrow::BorrowMut};

use image::{RgbImage, DynamicImage, ImageBuffer, Rgb};
use sdl2::{Sdl, VideoSubsystem, video::Window, render::{Canvas, Texture, TextureCreator}, pixels::{Color, PixelFormatEnum}, sys::SDL_Texture, event::Event, keyboard::Keycode, rect::Rect};

use crate::{world::{Camera}, math::{RectSize, IOResult}, ray::{Hittable, HitList}, geom::SdlTexture};

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

    pub fn render_world_to_image<T: Hittable>(&self, world: &HitList<T>, size: RectSize) -> IOResult<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let RectSize {width, height} = size;

        let mut image = DynamicImage::new_rgb8(width, height);
        let buffer = image.as_mut_rgb8().expect("Failed to convert image to rgb8");
    
    
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let u = (x as f64) / (width - 1) as f64;
            let v = (y as f64) / (height - 1) as f64;
            let ray = self.camera.cast_ray_at(u, v);
            pixel[0] = (ray.color(world.clone()).x() * 255.999) as u8;
            pixel[1] = (ray.color(world.clone()).y() * 255.999) as u8;
            pixel[2] = (ray.color(world.clone()).z() * 255.999) as u8;
        }
    
        if let Some(image) = image.flipv().as_rgb8() { 
            Ok(image.to_owned()) 
        } else { 
            Err("Failed to convert image to rgb8".into()) 
        }
        
    }
}

pub trait Drawable {
    fn draw(&self, renderer: &Renderer);
}
