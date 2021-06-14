use nalgebra as na;
use na::{ Vector3 };


pub struct Material {
   // Ambient, Diffuse, Specular, Mirror color
   pub ka: Vector3<f64>,
   pub kd: Vector3<f64>,
   pub ks: Vector3<f64>,
   pub km: Vector3<f64>,

   // Phong exponent
   pub phong_exp: f64
}


impl Material {
   pub fn new(ka: Vector3<f64>, kd: Vector3<f64>, ks: Vector3<f64>, km: Vector3<f64>, phong_exp: f64) -> Material
   {
      return Material { ka, kd, ks, km, phong_exp };
   }
}
