// for the capital I
#![allow(non_snake_case)]

use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Light;


pub struct PointLight {
   // Intensity (color)
   pub I: Vector3<f64>,

   // Position in space:
   pub p: Vector3<f64>
}


impl PointLight {
   pub fn new(I: Vector3<f64>, p: Vector3<f64>) -> PointLight
   {
      return PointLight { I, p };
   }
}


impl Light for PointLight {
   fn direction(&self, q: &Vector3<f64>, dir: &mut Vector3<f64>, max_t: &mut f64)
   {
      *dir = self.p - *q;
      *max_t = dir.norm();
      dir.normalize_mut();
   }

   fn get_intensity(&self) -> Vector3<f64>
   {
      return self.I;
   }
}
