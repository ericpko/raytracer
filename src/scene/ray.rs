use nalgebra as na;
use na::{ Vector3 };

// To see sibling modules
use crate::Camera;   // or: use super::Camera;

// use crate::geometry::Sphere;


pub struct Ray {
   pub origin: Vector3<f64>,
   // Not necessarily a unit-length direction vector.
   pub direction: Vector3<f64>
}


impl Ray {
   // Compute a viewing ray
   pub fn new(cam: &Camera, i: usize, j: usize, n_x: usize, n_y: usize) -> Ray 
   {
      // Get the scalar (weights) for the uvw coordinates:
      let u = -(cam.width / 2.0) + (cam.width * (i as f64 + 0.5) / n_x as f64);
      let v = (cam.height / 2.0) - (cam.height * (j as f64 + 0.5) / n_y as f64);
      let w = -cam.d;

      // Find the point <s> of (i, j) in the uvw camera frame coordinates
      // This is a projection onto the image plane:
      let s = cam.e + u*cam.u + v*cam.v + w*cam.w;

      // Get the viewing origin and direction:
      let o = cam.e;
      let dir = s - o;
      
      return Ray { origin: o, direction: dir }
   }

   
}
