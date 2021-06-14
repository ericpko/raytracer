// for the capital A, B, C, in intersect
#![allow(non_snake_case)]

use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Ray;
use crate::geometry::Object;
use crate::scene::Material;


pub struct Sphere {
   pub center: Vector3<f64>,
   pub radius: f64,
   pub material: Material
}


impl Sphere {
   pub fn new(c: &Vector3<f64>, r: f64, mat: Material) -> Sphere {
      Sphere { center: *c, radius: r, material: mat }
   }
}


impl Object for Sphere {
   fn intersect(&self, ray: &Ray, min_t: &f64, t: &mut f64, n: &mut Vector3<f64>) -> bool 
   {
      let mut hit = false;

      let c = self.center;
      let d = ray.direction;
      let e = ray.origin;

      let A: f64 = d.dot(&d);
      let B = 2. * d.dot(&(e - c));
      let C = (e - c).dot(&(e - c)) - self.radius.powf(2.0);

      let discriminant = B.powf(2.0) - (4. * A * C);
      if discriminant < 0. {
         // Then there is no solution
         return false;
      
      } else if discriminant == 0. {
         // Then there is one solution
         *t = -B / (2. * A);
         if *t >= *min_t {
            // The normal vector is the gradient of the sphere (pg 77): n = (p - c) / r
            *n = ((e + *t * d) - c) / self.radius;
            hit = true;
         }

      } else if discriminant > 0. {
         // Then there are two solutions. First calculate t
         let t_plus = (-B + discriminant.sqrt()) / (2. * A);
         let t_minus = (-B - discriminant.sqrt()) / (2. * A);
         let t_min = t_plus.min(t_minus);
         let t_max = t_plus.max(t_minus);

         if t_min >= *min_t {
            *t = t_min;
         } else if t_max >= *min_t {
            *t = t_max;
         }

         // Calculate the norm at the closest point of intersection:
         *n = ((e + *t * d) - c) / self.radius;
         hit = true;
      }

      return hit;
   }

   fn get_material(&self) -> &Material
   {
      return &self.material;
   }
}
