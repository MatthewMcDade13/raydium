extern crate sdl2;

use std::rc::Rc;
use std::sync::Arc;

use anyhow::Result;
use image::EncodableLayout;
use material::{Dielectric, Lambertian, Material, Metal};
use math::RectSize;
use ray::Hittable;
use render::{Drawable, SdlWindow, SdlRenderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use world::Camera;

use crate::geom::Sphere;
use crate::{ray::HitList, vec::Vec3};

mod geom;
mod material;
mod math;
mod ppm;
mod ray;
mod render;
mod vec;
mod world;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_SCATTER_DEPTH: u32 = 50;

// TODO :: Overall Cleanup

use eframe::egui;
fn main() -> anyhow::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(IMAGE_WIDTH as f32, IMAGE_HEIGHT as f32)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Raydium",
        options,
        Box::new(|_cc| Box::<Raydium>::default()),
    );
    Ok(())
}

struct Raydium;

impl Default for Raydium {
    fn default() -> Self {
        Raydium
    }
}

impl eframe::App for Raydium {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Raydium test gui");
            ui.horizontal(|ui| {
                let name_label = ui.label("Ayyyeee lmao");
            });
            if ui.button("Click me").clicked() {
                ui.label("Clicked!!!");
            }
        });
    }
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let window = SdlWindow::build_new(IMAGE_WIDTH, IMAGE_HEIGHT, "Raydium")?;
    let cam = Camera::new(
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
    );
    let renderer = SdlRenderer::new(window, cam);

    /**********************************************
     *
     */

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
    // world
    //     .0
    //     .push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    // world
    //     .0
    //     .push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let image = renderer.render_world_to_image(
        &world,
        RectSize {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
        },
        MAX_SCATTER_DEPTH,
    )?;
    // image = image.flipv();
    // image
    //     .save("_image.ppm")
    //     .expect("Failed to save image");r
    image.save("_image.ppm")?;

    /***********************************/

    let mut tex = renderer.create_static_texture(IMAGE_WIDTH, IMAGE_HEIGHT)?;
    let _ = tex.copy(image.as_bytes(), (IMAGE_WIDTH * 3) as usize)?;

    let mut epump = renderer.radwindow().events_mut();
    'running: loop {
        for event in epump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        renderer.clear_black();

        renderer.draw(&tex);

        renderer.swap_buffers();
    }
    Ok(())
}
