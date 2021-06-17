/**
 * This is the helper module.
 */
use std::{
   vec::Vec,
   fs::File,
   io::{ Write, BufWriter }
};
use nalgebra as na;
use na::{ Vector3 };
use serde_json::Value;

use crate::scene::{ 
   Camera,
   Light,
   PointLight,
   DirectionalLight,
   Material
};

use crate::geometry::{
   Object,
   Sphere,
   Plane
};



/**
 * There are two ways to write the image from a 1D array and all that matters
 * is that you need to stay consistent. The 2D raster display can be packed
 * inside a 1D array by columns or by rows. In this project I will choose to
 * represent the 2D raster display by row order inside a 1D array. It matters
 * how you loop over the 2D raster display. For instance, you can imagine
 * starting from (0, 0) in the bottom left corner and iterating across the x
 * values, then moving up one row in y and looping over the x values again.
 * This of course would make y in 0..n_y the outer loop. Then vice versa if you
 * were to imagine starting from the bottom corner and moving up all the way
 * to the top, then back down and continuing (x in 0..n_x outer loop). Changing
 * the loop order doesn't change the value you grab by accessing the 1D array,
 * but it does change the order when you write to the file. For instance, assume
 * you have your 1D image with the ball in the center stored by row-order of
 * the raster display, then whether or loop over x or y first, doesn't matter
 * if you were simply accessing the 1D array and looking at the color of that
 * pixel. However, the loop order does matter when you're also writing back
 * to a file.
 */
pub fn write_ppm(path: &str, rgb_image: &Vec<u8>, n_x: usize, n_y: usize, nchannels: usize)
{
   let f = File::create(path).expect(format!("Error creating file {}", path).as_str());
   let mut stream = BufWriter::new(&f);
   let header = format!("P6\n{} {}\n255\n", n_x, n_y);
   stream.write(header.as_bytes()).expect("Problem writing header.");


   // Not making any assumptions on how the image data is stored
   for subpxl in 0..nchannels * n_x * n_y {
      stream.write(&[rgb_image[subpxl] as u8]).expect("Error writing pixels.");
   }

   // 1D indices are 0, 1, 2, 3, 4, ..., n_x * n_y
   // for y in 0..n_y {
   //    for x in 0..n_x {
   //       let r = rgb_image[3 * (y * n_x + x) + 0] as u8;
   //       let g = rgb_image[3 * (y * n_x + x) + 1] as u8;
   //       let b = rgb_image[3 * (y * n_x + x) + 2] as u8;
   //       stream.write(&[r, g, b]).expect("Problem writing rgb");
   //    }
   // }

   // Equivalent to above. Changed the loop order and the indices (x * n_y + y)
   // for x in 0..n_x {
   //    for y in 0..n_y {
   //       let r = rgb_image[3 * (x * n_y + y) + 0] as u8;
   //       let g = rgb_image[3 * (x * n_y + y) + 1] as u8;
   //       let b = rgb_image[3 * (x * n_y + y) + 2] as u8;
   //       stream.write(&[r, g, b]).expect("Problem writing rgb");
   //    }
   // }

   stream.flush().expect("Error flushing buffer stream");
}


pub fn setup_camera(n_x: usize, n_y: usize, json: &Value, cam: &mut Camera)
{
   let cam_json = json.get("camera").unwrap();
   // let arr: Vec<f64> = cam_json["eye"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect();
   let eye: Vector3<f64> = Vector3::from_vec(cam_json["eye"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
   let focal_length = cam_json["focal_length"].as_f64().unwrap();

   let width = (n_x as f64) / (n_y as f64);
   let height = 1.0f64;

   let v: Vector3<f64> = Vector3::from_vec(cam_json["up"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect()).normalize();
   let w: Vector3<f64> = -Vector3::from_vec(cam_json["look"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect()).normalize();
   let u = v.cross(&w);

   *cam = Camera::new(eye, u, v, w, focal_length, width, height);
}

pub fn setup_objects(json: &Value, objects: &mut Vec<Box<dyn Object + Sync>>)
{
   let objects_json = json.get("objects").unwrap().as_array().unwrap();
   let mats_json = json.get("materials").unwrap().as_array().unwrap().to_vec();
   // let materials: Vec<Material> = Vec::default();

   for i in 0..objects_json.len() {
      if objects_json[i]["type"].as_str().unwrap() == "sphere" {
         let mat_name = objects_json[i]["material"].as_str().unwrap();
         let mat_idx = mats_json.iter().position(|j| j["name"].as_str().unwrap() == mat_name).unwrap();
         let ka: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["ka"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let kd: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["kd"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let ks: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["ks"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let km: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["km"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let phong_exp = mats_json[mat_idx]["phong_exponent"].as_f64().unwrap();
         
         let mat = Material::new(ka, kd, ks, km, phong_exp);

         let center: Vector3<f64> = Vector3::from_vec(objects_json[i]["center"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let radius: f64 = objects_json[i]["radius"].as_f64().unwrap();
         objects.push(Box::new(Sphere::new(&center, radius, mat)));
      
      } else if objects_json[i]["type"].as_str().unwrap() == "plane" {
         let mat_name = objects_json[i]["material"].as_str().unwrap();
         let mat_idx = mats_json.iter().position(|j| j["name"].as_str().unwrap() == mat_name).unwrap();
         let ka: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["ka"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let kd: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["kd"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let ks: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["ks"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let km: Vector3<f64> = Vector3::from_vec(mats_json[mat_idx]["km"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let phong_exp = mats_json[mat_idx]["phong_exponent"].as_f64().unwrap();
         
         let mat = Material::new(ka, kd, ks, km, phong_exp);

         let point: Vector3<f64> = Vector3::from_vec(objects_json[i]["point"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let normal: Vector3<f64> = Vector3::from_vec(objects_json[i]["normal"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect()).normalize();
         objects.push(Box::new(Plane::new(&point, &normal, mat)));
      }
   }
}


pub fn setup_lights(json: &Value, lights: &mut Vec<Box<dyn Light + Sync>>)
{
   let lights_json = json.get("lights").unwrap().as_array().unwrap();

   for i in 0..lights_json.len() {
      if lights_json[i]["type"].as_str().unwrap() == "directional" {
         let dir: Vector3<f64> = Vector3::from_vec(lights_json[i]["direction"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect()).normalize();
         let color: Vector3<f64> = Vector3::from_vec(lights_json[i]["color"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         lights.push(Box::new(DirectionalLight::new(color, dir)));
      
      } else if lights_json[i]["type"].as_str().unwrap() == "point" {
         let pos: Vector3<f64> = Vector3::from_vec(lights_json[i]["position"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         let color: Vector3<f64> = Vector3::from_vec(lights_json[i]["color"].as_array().unwrap().to_vec().iter().map(|x| x.as_f64().unwrap()).collect());
         lights.push(Box::new(PointLight::new(color, pos)));
      }
   }
}


pub fn setup_scene(n_x: usize, n_y: usize, json: &Value, cam: &mut Camera, lights: &mut Vec<Box<dyn Light + Sync>>, objects: &mut Vec<Box<dyn Object + Sync>>)
{
   setup_camera(n_x, n_y, json, cam);
   setup_lights(json, lights);
   setup_objects(json, objects);
}
