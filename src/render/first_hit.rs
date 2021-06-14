use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Ray;
use crate::geometry::Object;


pub fn first_hit(ray: &Ray, min_t: f64, objects: &Vec<Box<dyn Object>>, hit_id: &mut u32, t: &mut f64, n: &mut Vector3<f64>) -> bool
{
   let mut hit = false;

   // Make the temp variables before iterating through objects:
   let mut _t = 0.;
   let mut _n = Vector3::new(0., 0., 0.);
   let mut normal = Vector3::new(0.,0.,0.);
   let mut _hit_id = 0u32;
   let mut min_distance = f64::INFINITY;

   for (i, object) in objects.iter().enumerate() {
      if object.intersect(ray, &min_t, &mut _t, &mut _n) {
         // Then we have an intersection of ray and object i
         // Check the distance and update if the distance is smaller
         if _t < min_distance {
            min_distance = _t;
            normal = _n;
            _hit_id = i as u32;
            hit = true;
         }
      }
   }

   if hit {
      *t = min_distance;
      *n = normal;
      *hit_id = _hit_id;
   }


   return hit;
}
