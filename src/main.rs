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

         // Check if the ray intersects with the sphere
         // let mut t = f64::INFINITY;
         // let mut n: Vector3<f64> = Vector3::new(0., 0., 0.);
         // for object in &objects {
         //    if object.intersect(&ray, &1.0, &mut t, &mut n) {
         //       rgb_image[3 * (j * N_X + i) + 0] = 173u8;
         //       rgb_image[3 * (j * N_X + i) + 1] = 216u8;
         //       rgb_image[3 * (j * N_X + i) + 2] = 230u8;
         //    } else {
         //       rgb_image[3 * (j * N_X + i) + 0] = 170u8;
         //       rgb_image[3 * (j * N_X + i) + 1] = 240u8;
         //       rgb_image[3 * (j * N_X + i) + 2] = 209u8;
         //    }
         // }
         
         // Set the color
         rgb_image[3 * (j * N_X + i) + 0] = rgb[0] as u8;
         rgb_image[3 * (j * N_X + i) + 1] = rgb[1] as u8;
         rgb_image[3 * (j * N_X + i) + 2] = rgb[2] as u8;
      }
   }


   write_ppm("rgb.ppm", &rgb_image, N_X, N_Y, 3);
}
