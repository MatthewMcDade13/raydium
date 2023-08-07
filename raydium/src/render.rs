use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Instant,
};

use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use rand::{Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    sys::SDL_Texture,
    video::Window,
    Sdl, VideoSubsystem,
};

use crate::{
    geom::SdlTexture,
    math::{clamp, IOResult, RectSize},
    ray::{HitList, Hittable},
    vec::Vec3,
    world::Camera,
};

pub struct SdlWindow {
    ctx: Sdl,
    video: VideoSubsystem,
    canvas: RefCell<Canvas<sdl2::video::Window>>,
    event_pump: RefCell<sdl2::EventPump>,
}

pub struct SdlRenderer {
    window: SdlWindow,
    camera: Camera,
    texture_creator: Rc<TextureCreator<sdl2::video::WindowContext>>,
}

impl SdlWindow {
    pub fn build_new(
        width: u32,
        height: u32,
        title: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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
        Ok(SdlWindow {
            ctx: sdl.clone(),
            video,
            canvas: RefCell::new(canvas),
            event_pump: RefCell::new(sdl.event_pump()?),
        })
    }

    pub fn events(&self) -> Ref<sdl2::EventPump> {
        self.event_pump.borrow()
    }
    pub fn events_mut(&self) -> RefMut<sdl2::EventPump> {
        self.event_pump.borrow_mut()
    }
}

impl SdlRenderer {
    const DEFAULT_NUM_SAMPLES: u32 = 100;

    pub fn new(window: SdlWindow, camera: Camera) -> Self {
        let c = {
            let c = window.canvas.borrow();
            let tc = Rc::new(c.texture_creator());
            tc
        };

        Self {
            window,
            camera,
            texture_creator: c,
        }
    }

    pub const fn radwindow(&self) -> &SdlWindow {
        &self.window
    }
    pub fn canvas(&self) -> Ref<Canvas<sdl2::video::Window>> {
        self.window.canvas.borrow()
    }
    pub fn canvas_mut(&self) -> RefMut<Canvas<sdl2::video::Window>> {
        self.window.canvas.borrow_mut()
    }
    pub const fn window(&self) -> &SdlWindow {
        &self.window
    }

    pub fn create_static_texture(&self, width: u32, height: u32) -> IOResult<SdlTexture> {
        let t =
            self.texture_creator
                .create_texture_static(PixelFormatEnum::RGB24, width, height)?;
        let st = SdlTexture::new(t, Rect::new(0, 0, width, height));
        Ok(st)
    }

    pub fn create_streaming_texture(&self, width: u32, height: u32) -> IOResult<SdlTexture> {
        let t =
            self.texture_creator
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
    // TODO :: Put this in World with the Drawable trait
    pub fn render_world_to_image<T: Hittable + Send + Sync>(
        &self,
        world: &HitList<T>,
        size: RectSize,
        scatter_depth: u32,
    ) -> IOResult<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let RectSize { width, height } = size;

        let mut image = DynamicImage::new_rgb8(width, height);
        let buffer = image
            .as_mut_rgb8()
            .expect("Failed to convert image to rgb8");

        {
            let start = Instant::now();
            println!("Start render");
            draw_frame_parallel(
                buffer,
                &self.camera,
                &world,
                Self::DEFAULT_NUM_SAMPLES,
                scatter_depth,
                size,
            );
            println!("End render: Elapsed: {:.2?}", start.elapsed());
        }

        if let Some(image) = image.flipv().as_rgb8() {
            Ok(image.to_owned())
        } else {
            Err("Failed to convert image to rgb8".into())
        }
    }

    pub fn draw<T: Drawable>(&self, drawable: &T) {
        drawable.draw(self);
    }
}

fn draw_frame_parallel<T: Hittable + Sync + Send>(
    buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    camera: &Camera,
    world: &HitList<T>,
    num_samples: u32,
    scatter_depth: u32,
    size: RectSize,
) {
    let RectSize { width, height } = size;

    buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let color = {
                let mut color = Vec3::zero();

                for _ in 0..num_samples - 1 {
                    let u = (x as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / (height - 1) as f64;

                    let ray = camera.cast_ray(u, v);
                    color = color + ray.color(world.clone(), scatter_depth);
                }
                color
            };

            write_color(pixel, &color, num_samples);
        });
}

fn write_color(pixel: &mut Rgb<u8>, color: &Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();
    pixel[0] = (255.0 * clamp(r, 0.0, 0.999)) as u8;
    pixel[1] = (255.0 * clamp(g, 0.0, 0.999)) as u8;
    pixel[2] = (255.0 * clamp(b, 0.0, 0.999)) as u8;
}

pub trait Drawable {
    fn draw(&self, renderer: &SdlRenderer);
}
