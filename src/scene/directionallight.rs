// for the capital I
#![allow(non_snake_case)]

use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Light;


pub struct DirectionalLight {
   // Intensity (color)
   pub I: Vector3<f64>,
   // Direction FROM light towards scene
   pub d: Vector3<f64>
}


impl DirectionalLight {
   pub fn new(I: Vector3<f64>, d: Vector3<f64>) -> DirectionalLight
   {
      return DirectionalLight { I, d };
   }
}


impl Light for DirectionalLight {
   fn direction(&self, _q: &Vector3<f64>, dir: &mut Vector3<f64>, max_t: &mut f64)
   {
      *dir = -self.d.normalize();
      *max_t = f64::INFINITY;
   }

   fn get_intensity(&self) -> Vector3<f64>
   {
      return self.I;
   }
}
