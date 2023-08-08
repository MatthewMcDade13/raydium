use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Instant,
};

use image::{DynamicImage, ImageBuffer, Rgb, RgbImage, Rgba};
use rand::{Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::{
    math::{clamp, IOResult, RectSize},
    ray::{HitList, Hittable},
    vec::Vec3,
    world::Camera,
};

pub mod defaults {
    pub const NUM_SAMPLES: u32 = 100;
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
}

#[derive(Clone, Debug)]
pub struct RayRenderer {
    camera: Camera,
    num_samples: u32,
}

impl Default for RayRenderer {
    fn default() -> Self {
        Self {
            camera: Camera::new(
                Vec3(-2.0, 2.0, 1.0),
                Vec3(0.0, 0.0, -1.0),
                Vec3(0.0, 1.0, 0.0),
                90.0,
                defaults::ASPECT_RATIO,
            ),
            num_samples: defaults::NUM_SAMPLES
        }
    }
}

impl RayRenderer {

        pub const fn new(camera: Camera, num_samples: u32) -> Self {
            Self { camera, num_samples }
        }

        // TODO :: Put this in World with the Drawable trait
        pub fn render_world_to_image<T: Hittable + Send + Sync>(
            &self,
            world: &HitList<T>,
            size: RectSize,
            scatter_depth: u32,
        ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
            let RectSize { width, height } = size;
    
            let mut image = DynamicImage::new_rgba8(width, height);
            let buffer = image
                .as_mut_rgba8()
                .expect("Failed to convert image to rgb8");
    
            {
                let start = Instant::now();
                println!("Start render");
                draw_frame_parallel(
                    buffer,
                    &self.camera,
                    &world,
                    defaults::NUM_SAMPLES,
                    scatter_depth,
                    size,
                );
                println!("End render: Elapsed: {:.2?}", start.elapsed());
            }
            

           image.flipv().as_rgba8().expect("Failed to convert image to rgb8").clone()
        }

        pub const fn camera(&self) -> &Camera { &self.camera }
        pub const fn num_samples(&self) -> u32 { self.num_samples }
}



fn draw_frame_parallel<T: Hittable + Sync + Send>(
    buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
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

fn write_color(pixel: &mut Rgba<u8>, color: &Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();
    pixel[0] = (255.0 * clamp(r, 0.0, 0.999)) as u8;
    pixel[1] = (255.0 * clamp(g, 0.0, 0.999)) as u8;
    pixel[2] = (255.0 * clamp(b, 0.0, 0.999)) as u8;
    pixel[3] = 255;
}
