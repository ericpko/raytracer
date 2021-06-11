use nalgebra as na;
use na::{ Vector3 };


pub struct Camera {
   // Origin or "eye"
   pub e: Vector3<f64>,
   // Orthonormal frame such that -w is the viewing direction
   pub u: Vector3<f64>,
   pub v: Vector3<f64>,
   pub w: Vector3<f64>,
   // Image plane distance / focal length
   pub d: f64,
   // Width and height of the IMAGE PLANE
   pub width: f64,
   pub height: f64
}


impl Camera {
   pub fn new( e: Vector3<f64>, 
               u: Vector3<f64>, 
               v: Vector3<f64>, 
               w: Vector3<f64>, 
               d: f64, 
               width: f64, 
               height: f64) -> Camera 
   {
      Camera{e, u, v, w, d, width, height}
   }
}
