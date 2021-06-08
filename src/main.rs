// use nalgebra as na;
// use na::{Vector3};
use std::{
   vec::Vec,
   fs::File,
   io::{ Write, BufWriter }
};


fn render() {
   // Set the image width and height:
   const WIDTH: usize = 1024;
   const HEIGHT: usize = 768;

   // Create a temporary image:
   let mut framebuffer: Vec<f64> = vec![0.; 3 * WIDTH * HEIGHT];
   for row in 0..HEIGHT {
      for col in 0..WIDTH {
         framebuffer[3 * (row * WIDTH + col) + 0] = (row as f64) / (HEIGHT as f64);
         framebuffer[3 * (row * WIDTH + col) + 1] = (col as f64) / (WIDTH as f64);
         framebuffer[3 * (row * WIDTH + col) + 2] = 0.;
      }
   }

   // Write to ppm file:
   let f = File::create("out.ppm").expect("Something went wrong creating out.ppm");
   let mut stream = BufWriter::new(&f);
   let header = format!("P6\n{} {}\n255\n", WIDTH, HEIGHT);
   stream.write(header.as_bytes()).expect("Problem writing header.");
   for row in 0..HEIGHT {
      for col in 0..WIDTH {
         let r = (255. * 0.0f64.max(1.0f64.min(framebuffer[3 * (row * WIDTH + col) + 0]))) as u8;
         let g = (255. * 0.0f64.max(1.0f64.min(framebuffer[3 * (row * WIDTH + col) + 1]))) as u8;
         let b = (255. * 0.0f64.max(1.0f64.min(framebuffer[3 * (row * WIDTH + col) + 2]))) as u8;
         stream.write(&[r, g, b]).expect("Problem writing");
      }
   }

   stream.flush().expect("Problem flushing buffer stream.");
}


fn main() {
   render();
}
