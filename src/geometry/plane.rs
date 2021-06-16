use nalgebra as na;
use na::{ Vector3 };

use crate::scene::{ Ray, Material };
use crate::geometry::Object;


pub struct Plane {
   // Point on the plane
   pub point: Vector3<f64>,
   // Normal vector of the plane
   pub normal: Vector3<f64>,
   // Material of this Plane
   pub material: Material
}


impl Plane {
   pub fn new(p: &Vector3<f64>, n: &Vector3<f64>, mat: Material) -> Plane
   {
      return Plane { point: *p, normal: *n, material: mat }
   }
}


impl Object for Plane {
   // Check if ray intersected with plane
   fn intersect(&self, ray: &Ray, min_t: f64, t: &mut f64, n: &mut Vector3<f64>) -> bool
   {
      let mut hit = false;

      // Set up some vectors to represent the plane and line:
      let p = self.point;
      let e = ray.origin;
      let d = ray.direction;

      // Check if the direction of the line is perpendicular to the normal of
      // the plane. If this is the case, then the line does not intersect with plane.
      if d.dot(&self.normal) == 0. {
         return false;
      }

      // If we make it here, then we know that the ray intersects the plane
      // at some point t
      *t = self.normal.dot(&(p - e)) / self.normal.dot(&d);
      if *t >= min_t {
         *n = self.normal;
         hit = true;
      }

      return hit;
   }

   fn get_material(&self) -> &Material
   {
      return &self.material;
   }
}
