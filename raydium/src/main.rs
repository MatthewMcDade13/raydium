use std::io::Write;
mod ray;



const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() { 
    
    let out = raycast_image();

    println!("Hello, world!");
}


fn raycast_image() -> String {
    let mut out =String::new();
    out.push_str(&format!("P3\n{:?} {:?}\n255\n", IMAGE_WIDTH.to_string(), IMAGE_HEIGHT.to_string()));


    out
}