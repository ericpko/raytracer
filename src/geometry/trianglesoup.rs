// TODO: Not implemented. To let the compiler see this file, uncomment the lines in <geometry.rs>

use nalgebra as na;
use na::{ Vector3 };

use crate::scene::{ Ray, Material };
use crate::geometry::Object;


pub struct TriangleSoup {
   triangles: Vec<dyn Object + Sync>,
   material: Material
}


impl TriangleSoup {
   pub fn new(triangles: Vec<dyn Object + Sync>, material: Material) -> TriangleSoup {
      return TriangleSoup{triangles, material};
   }
}


impl Object for TriangleSoup {

}
