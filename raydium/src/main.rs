use eframe::epaint::ColorImage;
use image::ImageBuffer;
use rad::world::{Camera, CameraInfo};
use rand::Rng;
use std::sync::Arc;

use image::Rgba;
use log::*;
use poll_promise::Promise;
use rad::geom::Sphere;
use rad::material::{Dielectric, Lambertian, Material, Metal};
use rad::math::RectSize;
use rad::ray::HitList;
use rad::render::RayRenderer;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

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
            let app = Raydium::new(
                cc,
                RectSize {
                    width: 1200,
                    height: (1200. / (3. / 2.)) as u32,
                },
            );
            Box::new(app)
        }),
    );

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RenderState {
    Ready,
    Running,
    Finished,
    Progress(f32),
    RequestDraw,
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
    #[inline]
    fn render_world_to_image(&self) -> RaytraceFrame {
        RaytraceFrame(
            self.this
                .render_world_to_image(self.world.as_ref(), self.surface_size),
        )
    }
}

struct Raydium {
    renderer: Arc<RayRendererAsync>,
    render_state: RenderState,
    display_texture: Option<egui::TextureHandle>,
    render_rx: Option<Promise<egui::TextureHandle>>,
}
impl Raydium {
    pub fn new(_cc: &eframe::CreationContext<'_>, surface_size: RectSize) -> Self {
        const BEGIN_STATE: RenderState = RenderState::Ready;

        let world = Self::random_scene();
        let render_state = BEGIN_STATE;
        let look_from = Vec3(13.0, 2.0, 3.0);
        let look_at = Vec3::zero();
        let renderer = Arc::new(RayRendererAsync {
            this: RayRenderer::new(Camera::with_info(&CameraInfo {
                look_from,
                look_at,
                vert_up: Vec3(0., 1., 0.),
                vert_fov: 20.,
                aspect_ratio: 3.0 / 2.0,
                aperture: 0.1,
                focus_dist: 10., //(look_from - look_at).len(),
                samples_per_pixel: 500,

                ..Default::default()
            })),
            world: world.clone(),
            surface_size,
        });

        //renderer.do_send(DrawWorld);

        Self {
            renderer,
            render_state,
            display_texture: None,
            render_rx: None,
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
                    let ctx = ctx.clone();
                    let receiver = Promise::spawn_thread("Raydium Render", move || {
                        let image = renderer.render_world_to_image().0;
                        let pixels = image.as_flat_samples();
                        let size = [image.width() as _, image.height() as _];

                        let texture = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                        ctx.load_texture("Raycast Image", texture, Default::default())
                    });
                    self.render_rx = Some(receiver);
                }
            }

            ui.label(format!("{:?}", self.renderer.this.camera()));
            ui.label(format!("{:?}", self.renderer.this));
        });
    }

    fn random_scene() -> Arc<HitList<Sphere>> {
        let mut world = HitList::new();

        let ground_mat = Arc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5)));
        world.0.push(Arc::new(Sphere::new(
            ground_mat,
            Vec3(0., -1000., 0.),
            1000.0,
        )));

        let mut rng = rand::thread_rng();
        for i in -11..11 {
            for j in -11..11 {
                let choose_mat = rng.gen_range(0.0..1.0);
                let center = Vec3(
                    i as f64 + 0.9 * rng.gen_range(0.0..1.0),
                    0.2,
                    j as f64 + 0.9 * rng.gen_range(0.0..1.0),
                );

                if (center - Vec3(4., 0.2, 0.)).len() > 0.9 {
                    let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                        let albedo = Vec3::new_rand() * Vec3::new_rand();
                        Arc::new(Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        let albedo = Vec3::new_rand_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        Arc::new(Dielectric::new(1.5))
                    };
                    world
                        .0
                        .push(Arc::new(Sphere::new(sphere_material, center, 0.2)))
                }
            }
        }
        let mat1 = Arc::new(Dielectric::new(1.5));
        world
            .0
            .push(Arc::new(Sphere::new(mat1, Vec3(0., 1., 0.), 1.0)));
        let mat2 = Arc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)));
        world
            .0
            .push(Arc::new(Sphere::new(mat2, Vec3(-4., 1., 0.), 1.)));
        let mat3 = Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0));
        world
            .0
            .push(Arc::new(Sphere::new(mat3, Vec3(4., 1., 0.), 1.)));
        Arc::new(world)
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
                self.display_texture = Some(image.clone());
                // self.display_texture =
                // Some(ctx.load_texture("Raycast Image", texture_image, Default::default()));
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
