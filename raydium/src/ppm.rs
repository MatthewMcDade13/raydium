use crate::vec::{Vec3, Color};
use std::io::Write;

pub struct Image {
    width: u32,
    height: u32,
    ncols: u32,
    nrows: u32,
    data: Vec<Color>,
    metadata: String,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_col_row(width, height, height, width)
    }

    pub fn with_col_row(width: u32, height: u32, ncols: u32, nrows: u32) -> Self {
        let data = Vec::with_capacity((width * height) as usize);
        let metadata = format!("P3\n{} {}\n255\n", ncols, nrows);  
        
        Self { width, height, ncols, nrows, data, metadata }
    }

    pub const fn width(&self) -> u32 { self.width }
    pub const fn height(&self) -> u32 { self.height }
    pub const fn ncols(&self) -> u32 { self.ncols }
    pub const fn nrows(&self) -> u32 { self.nrows }

    pub fn push(&mut self, color: Color) {
        self.data.push(color);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn read_to_file(&self, filename: &str) -> std::io::Result<()> {

        let mut file = std::fs::File::create(filename)?;
        let mut s = String::with_capacity((self.width * self.height * 3) as usize);

        s.push_str(&self.metadata);

        for p in self.data.iter() {
            let pix = format!("{} {} {}\n", 
                p.x() as i32, 
                p.y() as i32,
                p.z() as i32
            );
            s.push_str(&pix);
        }

        file.write_all(s.as_bytes())?;
        Ok(())
    }
}





