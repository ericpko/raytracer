// for the capital I
#![allow(non_snake_case)]

use nalgebra as na;
use na::{ Vector3 };

use crate::scene::Ray;
use crate::geometry::Object;
use crate::scene::Light;
use crate::render::first_hit;


pub fn blinn_phong_shading(ray: &Ray, hit_id: usize, t: &f64, n: &Vector3<f64>,
                           objects: &Vec<Box<dyn Object>>,
                           lights: &Vec<Box<dyn Light>>) -> Vector3<f64>
{
   // Initialize some variables for readability:
   let mat = objects[hit_id].get_material();
   let kd = mat.kd;
   let ks = mat.ks;
   let ka = mat.ka;
   let phong_exp = mat.phong_exp;

   // Initialize pixel color with intensity of ambient light 
   let mut rgb = Vector3::new(ka[0] * 0.1, ka[1] * 0.1, ka[2] * 0.1);

   // Set up vectors for readability and notation:
   let e = ray.origin;
   let d = ray.direction;

   // Find the point <P> of intersection between the viewing ray and the surface:
   let P = e + *t * d;
   let mut sray: Ray = Ray{origin: P, direction: Vector3::new(0.,0.,0.)};                      // shadow ray
   sray.origin = P;

   // Iterate through every light and calculate all the vectors needed
   // to set the pixel color. Since light obeys the superposition principle,
   // we sum all the light, then clip if required:
   let v = (P + -d).normalize();     // v only depends on the viewing ray (not the light source)
   let mut l = Vector3::new(0.,0.,0.);
   // let mut h = Vector3::new(0.,0.,0.);
   for light in lights.iter() {
      // For each light, find the vectors l, and h, and set the shadow ray direction:
      let mut max_t = 0.;
      light.direction(&P, &mut l, &mut max_t);
      sray.direction = l;

      // Now we need to check if we're in a shadow.
      // (i.e. can we see the light source from <P>?)
      let mut s_hit_id: u32 = 0;
      let mut s_t: f64 = 0.;
      let mut s_n = Vector3::new(0.,0.,0.);
      if !first_hit(&sray, 1.0e-6, objects, &mut s_hit_id, &mut s_t, &mut s_n) || s_t >= max_t {
         // Then we're not in the shadow, so we can add the diffuse and specular components
         // NOTE: don't forget the s_t >= max_t because we might hit an object,
         // but that object might be above the light source, so we still want
         // that light. Without the <or> all our images are dimmer!
         let h = (v + l).normalize();      // only need to compute h if we're not in a shadow
         
         // Add the diffuse component (Lambertian shading) to the pixel color
         let I = light.get_intensity();
         rgb += Vector3::new(kd[0] * I[0], kd[1] * I[1], kd[2] * I[2]) * f64::max(0.0, n.dot(&l));
         // Add the specular component (Blinn-Phong shading) to the pixel color
         rgb += Vector3::new(ks[0] * I[0], ks[1] * I[1], ks[2] * I[2]) * f64::powf(f64::max(0.0, n.dot(&h)), phong_exp);
      }
   }

   return rgb;
}
