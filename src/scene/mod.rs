/**
 * This is just the module for scene. <scene> is a module. 
 * In rust, pretty much every .rs file is a module.
 * https://stackoverflow.com/questions/22596920/split-a-module-across-several-files
 * 
 * http://www.sheshbabu.com/posts/rust-module-system/
 */

pub mod camera;
pub use self::camera::Camera;

pub mod ray;
pub use self::ray::Ray;

pub mod pointlight;
pub use self::pointlight::PointLight;

pub mod directionallight;
pub use self::directionallight::DirectionalLight;

pub mod material;
pub use self::material::Material;




use nalgebra as na;
use na::{ Vector3 };


pub trait Light {
   // Input:
   //    q:  3D query point in space
   // Output:
   //    d:  3D direction from point towards light
   //    max_t:   parametric distance from q along dir to light (may be inf)
   fn direction(&self, q: &Vector3<f64>, dir: &mut Vector3<f64>, max_t: &mut f64);
   fn get_intensity(&self) -> Vector3<f64>;
}
