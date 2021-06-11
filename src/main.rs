use nalgebra as na;
use na::{ Vector3 };

mod scene;
use scene::Camera;
use scene::Ray;

mod helper;
use helper::write_ppm;

mod geometry;
use geometry::Sphere;


fn main() {
   println!("This is main.rs.\nTo run a step from raytracer, use:\ncargo run --bin <step>");

   // Set the image width and height in pixels:
   const N_X: usize = 1024;      // width
   const N_Y: usize = 800;       // height

   // Initialize the image:
   let mut rgb_image: Vec<u8> = vec![0; 3 * N_X * N_Y];

   // Setup the Camera:
   let eye = Vector3::new(0.0, 0.0, 4.0);
   let focal_length = 3.0f64;
   let width: f64 = 1.28;              // n_x / n_y
   let height = 1.0f64;
   let mut v: Vector3<f64> = Vector3::new(0.0, 1.0, 0.0);
   v.normalize_mut();
   let mut w: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);
   w.normalize_mut();
   let u = v.cross(&w);
   let cam = Camera::new(eye, u, v, w, focal_length, width, height);

   // Setup a sphere with center the origin and radius 0.5:
   let sphere = Sphere::new(&Vector3::new(0.0, 0.0, 0.0), 0.5);

   // Iterate over each pixel (i, j) = (x, y) = (col, row):
   for j in 0..N_Y 
   {
      for i in 0..N_X 
      {
         // Compute the viewing ray:
         let ray = Ray::new(&cam, i, j, N_X, N_Y);

         // Check if the ray intersects with the sphere
         let mut t = f64::INFINITY;
         let mut n: Vector3<f64> = Vector3::new(0., 0., 0.);
         if sphere.intersect(&ray, &1.0, &mut t, &mut n) {
            rgb_image[3 * (j * N_X + i) + 0] = 173u8;
            rgb_image[3 * (j * N_X + i) + 1] = 216u8;
            rgb_image[3 * (j * N_X + i) + 2] = 230u8;
         } else {
            rgb_image[3 * (j * N_X + i) + 0] = 170u8;
            rgb_image[3 * (j * N_X + i) + 1] = 240u8;
            rgb_image[3 * (j * N_X + i) + 2] = 209u8;
         }
      }
   }


   write_ppm("rgb.ppm", &rgb_image, N_X, N_Y, 3);
}
