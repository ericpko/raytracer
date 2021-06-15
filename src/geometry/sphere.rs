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
      let C = (e - c).dot(&(e - c)) - f64::powf(self.radius, 2.0);

      let discriminant = B.powf(2.0) - (4. * A * C);
      if discriminant < 0. {
         // Then there is no solution (ray didn't intersect sphere)
         return hit;
      // }

      // // If we make it here, then we definitely hit something
      // hit = true;

      // // Find the smallest value of t (there could be two solutions (if discr < 0)):
      // let t_plus = (-B + discriminant.sqrt()) / (2. * A);
      // let t_minus = (-B - discriminant.sqrt()) / (2. * A);
      // // If t < 0, then ray started inside sphere so clamp t to zero:
      // *t = f64::max(f64::min(t_plus, t_minus), 0.0);

      // // Set the normal vector at the point of intersection:
      // *n = ((e + *t * d) - c) / self.radius;

////////////////////////////////////////////////////////////////////////////////////////////////////
      
      } else if discriminant >= -0.00006 && discriminant <= 0.00006 {
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
         let t_min = f64::min(t_plus, t_minus);
         let t_max = f64::max(t_plus, t_min);

         // println!("min_t: {}\nt_min: {}\nt_max: {}", *min_t, t_min, t_max);
         if t_min >= *min_t {
            *t = t_min;
            hit = true;
         } else if t_max >= *min_t {
            *t = t_max;
            hit = true;
         }

         // Calculate the norm at the closest point of intersection:
         *n = ((e + *t * d) - c) / self.radius;
         // hit = true;
      }

      return hit;
   }

   fn get_material(&self) -> &Material
   {
      return &self.material;
   }
}
