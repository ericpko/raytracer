use nalgebra as na;
use na::{ Vector3 };


// Reflect an incoming ray
// Inputs:
//    in: incoming _unit_ ray direction
//    n: surface _unit_ ray direction
// Returns outward _unit_ ray direction
pub fn reflect(dir: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64>
{
   let r = dir - 2. * dir.dot(n) * n;     // reflection direction
   return r.normalize();
}
