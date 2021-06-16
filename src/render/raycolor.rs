use nalgebra as na;
use na::{ Vector3 };

use crate::scene::{ Ray, Light };
use crate::render::{ first_hit, blinn_phong_shading, reflect };
use crate::geometry::Object;



pub fn raycolor(  ray: &Ray, 
                  min_t: f64, 
                  objects: &Vec<Box<dyn Object + Sync>>, 
                  lights: &Vec<Box<dyn Light + Sync>>, 
                  n_recursive_calls: usize, 
                  rgb: &mut Vector3<f64>) -> bool
{
   if n_recursive_calls > 4 {
      return false;
   }

   // Initialize variables:
   let mut n = Vector3::new(0.,0.,0.);
   let mut t = 0.0f64;
   let mut hit_id = 0usize;

   // Check if we hit an object. If not, then return false:
   if !first_hit(&ray, min_t, objects, &mut hit_id, &mut t, &mut n) {
      return false;
   }

   // If we made it here, then the viewing ray has intersected an object (hit).
   // Now we evauluate the shading model and set the pixel color:
   *rgb = blinn_phong_shading(ray, hit_id, &t, &n, objects, lights);

   // Now we add ideal specular reflection and mirror reflection (pg 87).
   // We need to set up a new mirror ray (mray):
   // Get the real intersection point between the viewing ray and the surface
   let e = ray.origin;
   let d = ray.direction;
   let mray = Ray{origin: e + t * d, direction: reflect(&d.normalize(), &n.normalize())};

   // Now we can recursively add to our rgb pixel color. 
   // s = 1e-5 is our fudge factor to move off the surface
   let mut mrgb = Vector3::new(0.,0.,0.);
   if raycolor(&mray, 1.0e-6, objects, lights, n_recursive_calls + 1, &mut mrgb) {
      // Then we can update the rgb value. Componenet wise multiplication
      let mat = objects[hit_id].get_material();
      *rgb += Vector3::new(mat.km[0] * mrgb[0], mat.km[1] * mrgb[1], mat.km[2] * mrgb[2]);
   }


   return true;
}
