/**
 * This is the helper module.
 */
use std::{
   vec::Vec,
   fs::File,
   io::{ Write, BufWriter }
};
use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Camera;
use crate::scene::Light;
use crate::scene::PointLight;
use crate::scene::DirectionalLight;
use crate::scene::Material;
use crate::geometry::Object;
use crate::geometry::Sphere;
use crate::geometry::Plane;


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


pub fn setup_camera() -> Camera
{
   let eye = Vector3::new(0.0, 0.0, 5.0);
   let focal_length = 3.0f64;
   let width: f64 = 1.28;              // n_x / n_y
   let height = 1.0f64;
   let mut v: Vector3<f64> = Vector3::new(0.0, 1.0, 0.0);
   v.normalize_mut();
   let mut w: Vector3<f64> = -Vector3::new(0.0, 0.0, -1.0);       // double negative?
   w.normalize_mut();
   let u = v.cross(&w);
   let cam = Camera::new(eye, u, v, w, focal_length, width, height);

   return cam;
}

pub fn create_objects() -> std::vec::Vec<Box<dyn Object>>
{
   let orange_pastic = Material::new(
      Vector3::new(1., 0.7, 0.2),
      Vector3::new(1., 0.7, 0.2),
      Vector3::new(0.8, 0.8, 0.8),
      Vector3::new(0.05, 0.05, 0.05),
      1000.0
   );
   let lambertian_blue = Material::new(
      Vector3::new(0.2, 0.3, 0.8),
      Vector3::new(0.2, 0.3, 0.8),
      Vector3::new(0.1, 0.1, 0.1),
      Vector3::new(0.3, 0.3, 0.3),
      20.0
   );

   let sphere = Sphere::new(&Vector3::new(0.0, 0.0, 0.0), 0.5, orange_pastic);
   let plane = Plane::new(&Vector3::new(0., -0.5, 0.), &Vector3::new(0., 1., 0.), lambertian_blue);

   let mut objects: Vec<Box<dyn Object>> = Vec::new();
   objects.push(Box::new(sphere));
   objects.push(Box::new(plane));

   return objects;
}


pub fn create_lights() -> Vec<Box<dyn Light>>
{
   let color = Vector3::new(0.8, 0.8, 0.8);

   let pointlight = PointLight::new(color, Vector3::new(-10., 20., 10.));

   let mut dir = Vector3::new(0., 0., -1.);
   dir.normalize_mut();
   let directionallight = DirectionalLight::new(color, dir);

   let mut lights: Vec<Box<dyn Light>> = Vec::new();
   lights.push(Box::new(pointlight));
   lights.push(Box::new(directionallight));

   return lights;
}
