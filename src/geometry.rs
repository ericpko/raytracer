/**
 * This is just a module called geometry and the files inside
 * the geometry directory are also modules, but submodules of this
 * module.
 * 
 * This is the same thing as creating a mod.rs file inside of the 
 * geometry directory (similar to scene/mod.rs). This method is the 
 * now preferred (Rust 2018) way of creating modules so you don't end
 * up with a bunch of mod.rs files.
 */
pub mod sphere;
pub use self::sphere::Sphere;

pub mod plane;
pub use self::plane::Plane;

pub mod triangle;
pub use self::triangle::Triangle;

// pub mod trianglesoup;
// pub use self::trianglesoup::TriangleSoup;



use nalgebra as na;
use na::{ Vector3 };
use crate::scene::{ Ray, Material };



pub trait Object {
   fn intersect(&self, ray: &Ray, min_t: f64, t: &mut f64, n: &mut Vector3<f64>) -> bool;
   fn get_material(&self) -> &Material;
}
