use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Ray;
use crate::geometry::Object;


pub fn first_hit(ray: &Ray, min_t: f64, objects: &Vec<Box<dyn Object + Sync>>, hit_id: &mut usize, t: &mut f64, n: &mut Vector3<f64>) -> bool
{
   let mut hit = false;

   // Make some temp variables before iterating through each object:
   let mut _t = 0.0;
   let mut _n = Vector3::new(0.,0.,0.);
   let mut min_distance = f64::INFINITY;

   for (i, object) in objects.iter().enumerate() {
      if object.intersect(ray, min_t, &mut _t, &mut _n) {
         // Then we have an intersection of ray and object i
         hit = true;

         // Update <t> and <n> and <hit_id> if we hit a closer object
         if _t < min_distance {
            min_distance = _t;
            *t = _t;
            *n = _n;
            *hit_id = i as usize;
         }
      }
   }


   return hit;
}




// Equivalent to above, maybe more readable?
// pub fn first_hit(ray: &Ray, min_t: f64, objects: &Vec<Box<dyn Object + Sync>>, hit_id: &mut usize, t: &mut f64, n: &mut Vector3<f64>) -> bool
// {
//    let mut hit = false;

//    // Make the temp variables before iterating through objects:
//    let mut normal = Vector3::new(0.,0.,0.);
//    let mut min_distance = f64::INFINITY;

//    for (i, object) in objects.iter().enumerate() {
//       if object.intersect(ray, min_t, t, n) {
//          // Then we have an intersection of ray and object i
//          // Check the distance and update if the distance is smaller
//          if *t < min_distance {
//             min_distance = *t;
//             normal = *n;
//             *hit_id = i as usize;
//             hit = true;
//          }
//       }
//    }

//    if hit {
//       *t = min_distance;
//       *n = normal;
//    }


//    return hit;
// }
