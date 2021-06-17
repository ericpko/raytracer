// for the capital letters
#![allow(non_snake_case)]

use nalgebra as na;
use na::{ Vector3 };

use crate::scene::{ Ray, Material };
use crate::geometry::Object;



pub struct Triangle {
   pub corners: (Vector3<f64>, Vector3<f64>, Vector3<f64>),
   pub material: Material
}


impl Triangle {
   pub fn new(corners: (Vector3<f64>, Vector3<f64>, Vector3<f64>), material: Material) -> Triangle {
      return Triangle{ corners, material };
   }
}


impl Object for Triangle {
   fn intersect(&self, ray: &Ray, min_t: f64, t: &mut f64, n: &mut Vector3<f64>) -> bool
   {
      // Solution found on page 79 in textbook
      let mut hit = false;

      // Destructure corners and init variables
      let (P, Q, R) = self.corners;
      let PQ = Q - P;
      let PR = R - P;
      let dir = ray.direction;
      let eye = ray.origin;

      // Get the normal of the two vectors in the plane of the triangle:
      let normal = PQ.cross(&PR);

      // Check if the ray intersects with the plane made by the triangle:
      if dir.dot(&normal) == 0. {
         return hit;
      }

      // If we make it here, then we know that the ray intersects the plane
      // at some point <t>:
      *t = normal.dot(&(P - eye)) / normal.dot(&dir);
      if *t < min_t {
         return hit;
      }

      ////////////////////////////////////////////////////////////////////////
      // Now we check if the intersection point is in the triangle:
      // Column Vector (P - Q):
      let a = -PQ[0];
      let b = -PQ[1];
      let c = -PQ[2];
      // Column Vector (P - R):
      let d = -PR[0];
      let e = -PR[1];
      let f = -PR[2];
      // Column Vector dir
      let g = dir[0];
      let h = dir[1];
      let i = dir[2];
      
      // Column vector y      Ax = y:
      let j = P[0] - eye[0];
      let k = P[1] - eye[1];
      let l = P[2] - eye[2];

      let ei_minus_hf = e*i - h*f;
      let gf_minus_di = g*f - d*i;
      let dh_minus_eg = d*h - e*g;

      let ak_minus_jb = a*k - j*b;
      let jc_minus_al = j*c - a*l;
      let bl_minus_kc = b*l - k*c;

      // Get the determinant of matrix A:
      let M = a*ei_minus_hf + b*gf_minus_di + c*dh_minus_eg;

      // Solve for the values of x = <beta, gamma, t> in the system Ax = y
      let beta = (j*ei_minus_hf + k*gf_minus_di + l*dh_minus_eg) / M;
      let gamma = (i*ak_minus_jb + h*jc_minus_al + g*bl_minus_kc) / M;

      if beta >= 0. && gamma >= 0. && beta + gamma <= 1. {
         *n = normal.normalize();
         hit = true;
      }

      return hit;
   }


   fn get_material(&self) -> &Material
   {
      return &self.material;
   }
}
