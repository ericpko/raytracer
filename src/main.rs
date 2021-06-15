use nalgebra as na;
use na::{ Vector3 };


mod scene;
use scene::Camera;
use scene::Ray;

mod helper;
use helper::{ write_ppm, setup_camera, create_objects, create_lights };

mod geometry;

mod render;
use render::raycolor;



fn main() {
   println!("This is main.rs.\nTo run a step from raytracer, use:\ncargo run --bin <step>");

   // Set the image width and height in pixels:
   const N_X: usize = 1024;      // width
   const N_Y: usize = 800;       // height

   // Initialize the image:
   let mut rgb_image: Vec<u8> = vec![0; 3 * N_X * N_Y];

   // Lights, camera, ACTION
   // Create lights:
   let lights = create_lights();
   // Create objects:
   let objects = create_objects();
   // Setup the Camera:
   let cam = setup_camera();

   // TODO: parallelize
   // Iterate over each pixel (i, j) = (x, y) = (col, row):
   for j in 0..N_Y 
   {
      for i in 0..N_X 
      {
         // Compute the viewing ray:
         let ray = Ray::new(&cam, i, j, N_X, N_Y);

         // Set the background color by default:
         let mut rgb = Vector3::new(0., 0., 0.);

         // Shoot a ray and collect the color:
         raycolor(&ray, 1., &objects, &lights, 0, &mut rgb);
         
         // Define an anonymous function (lambda function or closure in Rust)
         // to clamp the rgb values after adding lights from blinn_phong_shading
         let clamp = |s: f64| -> f64 {
            // return 0.0f64.max(s.min(1.0));
            return f64::max(f64::min(s, 1.0), 0.0);
         };

         // Set the color
         rgb_image[3 * (j * N_X + i) + 0] = (255.0 * clamp(rgb[0])) as u8;
         rgb_image[3 * (j * N_X + i) + 1] = (255.0 * clamp(rgb[1])) as u8;
         rgb_image[3 * (j * N_X + i) + 2] = (255.0 * clamp(rgb[2])) as u8;
      }
   }


   write_ppm("rgb.ppm", &rgb_image, N_X, N_Y, 3);
}
