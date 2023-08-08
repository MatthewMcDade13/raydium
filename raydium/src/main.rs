

use std::sync::Arc;
use eframe::epaint::ColorImage;
use image::ImageBuffer;
use image::Rgb;
use image::Rgba;
use log::*;
use poll_promise::Promise;
use rad::geom::Sphere;
use rad::material::{Lambertian, Metal, Dielectric};
use rad::math::RectSize;
use rad::ray::HitList;
use rad::render::RayRenderer;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_SCATTER_DEPTH: u32 = 50;

// TODO :: Overall Cleanup

use eframe::egui;
use rad::vec::Vec3;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(IMAGE_WIDTH as f32, IMAGE_HEIGHT as f32)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Raydium",
        options,
        Box::new(|cc| {
            let app = Raydium::new(cc, RectSize { width: IMAGE_WIDTH, height: IMAGE_HEIGHT });
            Box::new(app)
        }),
    );
    
    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RenderState {
    Ready, Running, Finished, Progress(f32), RequestDraw,
}

impl Default for RenderState {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Clone)]
struct RaytraceFrame(ImageBuffer<Rgba<u8>, Vec<u8>>);

unsafe impl Send for RaytraceFrame {}
unsafe impl Sync for RaytraceFrame {}

struct RayRendererAsync {
    this: RayRenderer,
    world: Arc<HitList<Sphere>>,
    surface_size: RectSize,
}

impl RayRendererAsync {
    // fn draw_frame(&mut self) -> Result<RaytraceFrame, Box<dyn std::error::Error>> {
    //     {
    //         let mut s = self.render_state.write()?;
    //         *s = RenderState::Running;
    //     }

    //     let image = self.render_world_to_image();
        
    //     {
    //         let mut s = self.render_state.write()?;
    //         *s = RenderState::Finished;
    //     }
        
    //     Ok(image.clone())
    // }

    #[inline]
    fn render_world_to_image(&self) -> RaytraceFrame {
        RaytraceFrame(
            self.this.render_world_to_image(self.world.as_ref(), self.surface_size, MAX_SCATTER_DEPTH)
        )
    }

}



// impl Handler<DrawWorld> for RayRendererAsync {
//     type Result = ();

//     fn handle(&mut self, msg: DrawWorld, ctx: &mut Self::Context) -> Self::Result {



//     }
// }


struct Raydium {
    renderer: Arc<RayRendererAsync>,
    render_state: RenderState,
    display_texture: Option<egui::TextureHandle>,
    render_rx: Option<Promise<RaytraceFrame>>,
}

// unsafe impl Send for RayRendererAsync {}
// unsafe impl Sync for RayRendererAsync {}

impl Raydium { 
    pub fn new(_cc: &eframe::CreationContext<'_>, surface_size: RectSize) -> Self {

        const BEGIN_STATE: RenderState = RenderState::Ready;

        let world = Self::create_world();
        let render_state = BEGIN_STATE;
        let renderer = Arc::new(RayRendererAsync {
            this: RayRenderer::default(),
            world: world.clone(),
            surface_size,
        });
        
        //renderer.do_send(DrawWorld);
        
        Self {
            renderer,
            render_state,
            display_texture: None,
            render_rx: None
        }
    }

    fn display_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("Render Panel").show(ctx, |ui| { 
            let state_text = {
                let st = match self.render_state {
                    RenderState::Ready => "Ready".into(),
                    RenderState::Running => "Running".into(),
                    RenderState::Finished => "Finished".into(),
                    RenderState::Progress(percent) => format!("{}%", percent * 100.0),
                    RenderState::RequestDraw => "Requesting Draw".into(),
                };
                st
            };

            ui.label(format!("Render State: {}", state_text));
            if ui.button("Render Frame").clicked() {
                let rs = self.render_state;
                if rs == RenderState::Ready || rs == RenderState::Finished {
                    self.render_state = RenderState::Running;
                    let renderer = self.renderer.clone();
                    let receiver = Promise::spawn_thread("Raydium Render", move || {
                        renderer.render_world_to_image()
                    });         
                    self.render_rx = Some(receiver);
                }     
            }
        });
    }


    fn create_world() -> Arc<HitList<Sphere>> {
        
        let mut world = HitList::new();

        let mat_ground = Arc::new(Lambertian::new(Vec3(0.8, 0.8, 0.0)));
        let mat_center = Arc::new(Lambertian::new(Vec3(0.1, 0.2, 0.5)));
        let mat_left = Arc::new(Dielectric::new(1.5));
        let mat_right = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.0));

        world.0.push(Arc::new(Sphere::new(
            mat_ground,
            Vec3(0.0, -100.5, -1.0),
            100.0,
        )));
        world.0.push(Arc::new(Sphere::new(
            mat_center.clone(),
            Vec3(0.0, 0.0, -1.0),
            0.5,
        )));
        world.0.push(Arc::new(Sphere::new(
            mat_left.clone(),
            Vec3(-1.0, 0.0, -1.0),
            0.5,
        )));
        world.0.push(Arc::new(Sphere::new(
            mat_left.clone(),
            Vec3(-1.0, 0.0, -1.0),
            -0.4,
        )));
        world.0.push(Arc::new(Sphere::new(
            mat_right.clone(),
            Vec3(1.0, 0.0, -1.0),
            0.5,
        )));
        Arc::new(world)
    }
}

impl eframe::App for Raydium {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Some(ref prom) = self.render_rx {
            if let Some(image) = prom.ready() {
                
                let pixels = image.0.as_flat_samples();
                let size = [image.0.width() as _, image.0.height() as _];
                let texture_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                self.display_texture = Some(                    
                    ctx.load_texture("Raycast Image", texture_image, Default::default())
                );
                self.render_state = RenderState::Ready;
                self.render_rx = None;
            }
        } 

        self.display_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref texture) = self.display_texture {
                ui.image(texture, texture.size_vec2());
            } else {
                ui.label("No Image");
            }
        });
    }
}



// impl actix::Supervised for Raydium {
//     fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {}
// }

// fn _main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();

//     /**********************************************
//      *
//      */

//     // world
//     //     .0
//     //     .push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
//     // world
//     //     .0
//     //     .push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

//     let image = renderer.render_world_to_image(
//     Ok(())

//     /***********************************/
// }
