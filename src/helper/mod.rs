/**
 * This is the helper module.
 */
use std::{
   vec::Vec,
   fs::File,
   io::{ Write, BufWriter }
};


/**
 * There are two ways to write the image from a 1D array and all that matters
 * is that you need to stay consistent. The 2D raster display can be packed
 * inside a 1D array by columns or by rows. In this project I will choose to
 * represent the 2D raster display by row order inside a 1D array. It matters
 * how you loop over the 2D raster display. For instance, you can imagine
 * starting from (0, 0) in the bottom left corner and iterating across the x
 * values, then moving up one row in y and looping over the x values again.
 * This of course would make y in 0..n_y the outer loop. Then vice versa if you
 * were to imagine starting from the bottom corner and moving up all the way
 * to the top, then back down and continuing (x in 0..n_x outer loop). Changing
 * the loop order doesn't change the value you grab by accessing the 1D array,
 * but it does change the order when you write to the file. For instance, assume
 * you have your 1D image with the ball in the center stored by row-order of
 * the raster display, then whether or loop over x or y first, doesn't matter
 * if you were simply accessing the 1D array and looking at the color of that
 * pixel. However, the loop order does matter when you're also writing back
 * to a file.
 */
pub fn write_ppm(path: &str, rgb_image: &Vec<u8>, n_x: usize, n_y: usize, nchannels: usize)
{
   let f = File::create(path).expect(format!("Error creating file {}", path).as_str());
   let mut stream = BufWriter::new(&f);
   let header = format!("P6\n{} {}\n255\n", n_x, n_y);
   stream.write(header.as_bytes()).expect("Problem writing header.");


   // Not making any assumptions on how the image data is stored
   for subpxl in 0..nchannels * n_x * n_y {
      stream.write(&[rgb_image[subpxl] as u8]).expect("Error writing pixels.");
   }

   // 1D indices are 0, 1, 2, 3, 4, ..., n_x * n_y
   // for y in 0..n_y {
   //    for x in 0..n_x {
   //       let r = rgb_image[3 * (y * n_x + x) + 0] as u8;
   //       let g = rgb_image[3 * (y * n_x + x) + 1] as u8;
   //       let b = rgb_image[3 * (y * n_x + x) + 2] as u8;
   //       stream.write(&[r, g, b]).expect("Problem writing rgb");
   //    }
   // }

   // Equivalent to above. Changed the loop order and the indices (x * n_y + y)
   // for x in 0..n_x {
   //    for y in 0..n_y {
   //       let r = rgb_image[3 * (x * n_y + y) + 0] as u8;
   //       let g = rgb_image[3 * (x * n_y + y) + 1] as u8;
   //       let b = rgb_image[3 * (x * n_y + y) + 2] as u8;
   //       stream.write(&[r, g, b]).expect("Problem writing rgb");
   //    }
   // }

   stream.flush().expect("Error flushing buffer stream");
}
