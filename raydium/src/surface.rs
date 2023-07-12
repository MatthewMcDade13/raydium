use crate::vec::Vec3;

pub trait Renderable {
    fn draw_frame(&self) -> Vec<Vec3>;
}

pub struct Surface {
    aspect_ratio: f64,
    width: u32,
    height: u32,
    framebuffer: Vec<Vec3>,
}

impl Surface {
    pub fn new(aspect_ratio: f64, width: u32, height: u32) -> Self {
        Self {
            aspect_ratio,
            width,
            height,
            framebuffer: vec![Vec3::zero(); (width * height) as usize],
        }
    }

    pub const fn width(&self) -> u32 { self.width }
    pub const fn height(&self) -> u32 { self.height }
    pub const fn aspect_ratio(&self) -> f64 { self.aspect_ratio }
    
}

impl Renderable for Surface {
    fn draw_frame(&self) -> Vec<Vec3> {
        
    }
}