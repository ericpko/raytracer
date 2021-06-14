use nalgebra as na;
use na::{ Vector3, Vector4 };

use std::{
   vec::Vec,
   fs::File,
   io::{ Write, BufWriter }
};


struct Light {
   position: Vector3<f64>,
   intensity: f64
}

impl Light {
   fn new(p: Vector3<f64>, I: f64) -> Light
   {
      return Light { position: p, intensity: I };
   }
}

#[derive(Copy, Clone)]
struct Material {
   refractive_index: f64,
   albedo: Vector4<f64>,
   diffuse_color: Vector3<f64>,
   specular_exponent: f64
}

impl Material {
   fn new(refractive_index: f64, albedo: Vector4<f64>, diffuse_color: Vector3<f64>, specular_exponent: f64) -> Material
   {
      return Material { refractive_index, albedo, diffuse_color, specular_exponent };
   }
}

struct Sphere {
   center: Vector3<f64>,
   radius: f64,
   material: Material
}

impl Sphere {
   fn new(center: Vector3<f64>, radius: f64, material: Material) -> Sphere
   {
      return Sphere { center, radius, material }
   }
}


fn reflect(dir: &Vector3<f64>, N: &Vector3<f64>) -> Vector3<f64>
{
   let mut ddir = 2. * dir;
   ddir[0] = ddir[0] * N[0];
   ddir[1] = ddir[1] * N[1];
   ddir[2] = ddir[2] * N[2];

   return dir - ddir
}

fn refract(dir: &Vector3<f64>, N: &Vector3<f64>, eta_t: f64, eta_i: f64) -> Vector3<f64>
{
   return Vector3::new(0.,0.,0.);
}


fn ray_sphere_intersect(eye: &Vector3<f64>, dir: &Vector3<f64>, sphere: &Sphere, t0: &mut f64) -> bool
{


   return true;   
}


fn scene_intersect(eye: &Vector3<f64>, dir: &Vector3<f64>, spheres: &Vec<Sphere>, hit: &mut Vector3<f64>, N: &mut Vector3<f64>, mat: &mut Material) -> bool
{
   let mut sphere_dist = f64::INFINITY;
   for sphere in spheres {
      let mut dist_i = 0.;
      if ray_sphere_intersect(eye, dir, &sphere, &mut dist_i) {
         sphere_dist = dist_i;
         *hit = eye + dir * dist_i;
         *N = (*hit - sphere.center).normalize();
         *mat = sphere.material;
      }
   }
   return false;
}


fn ray_cast(rgb: &mut Vector3<f64>, eye: &Vector3<f64>, dir: &Vector3<f64>, spheres: &Vec<Sphere>, lights: &Vec<Light>, depth: usize)
{
   let mut point = Vector3::new(0.,0.,0.);
   let mut N = Vector3::new(0.,0.,0.);
   let mut mat = Material::new(1., Vector4::new(0.6,0.3,0.1,0.0), Vector3::new(0.4,0.4,0.3), 50.);
   if depth > 4 || !scene_intersect(eye, dir, spheres, &mut point, &mut N, &mut mat) {
      *rgb = Vector3::new(0.2, 0.7, 0.8);    // background color
   }

   let reflect_dir = reflect(dir, &N).normalize();
   let refract_dir = refract(dir, &N, mat.refractive_index, 1.).normalize();
   
   let mut rgb_reflect = Vector3::new(0.,0.,0.);
   ray_cast(&mut rgb_reflect, &point, &reflect_dir, spheres, lights, depth + 1);
   let mut rgb_refract = Vector3::new(0.,0.,0.);
   ray_cast(&mut rgb_refract, &point, &refract_dir, spheres, lights, depth + 1);

   let mut diffuse_light_intensity = 0.;
   let mut specular_light_intensity = 0.;
   for light in lights {
      let light_dir = (light.position - point).normalize();

      let mut shadow_pt = Vector3::new(0.,0.,0.);
      let mut trashnrm = Vector3::new(0.,0.,0.);
      let mut trashmat = Material::new(1., Vector4::new(0.6,0.3,0.1,0.0), Vector3::new(0.4,0.4,0.3), 50.);
      if scene_intersect(&point, &light_dir, spheres, &mut shadow_pt, &mut trashnrm, &mut trashmat) && 
               shadow_pt.norm() < (light.position - point).norm() {
                  continue;
      }
      let mut tmp1 = light_dir[0]*N[0] + light_dir[1]*N[1] + light_dir[2]*N[2];
      diffuse_light_intensity += light.intensity * 0.0f64.max(tmp1);
      let tmp2 = -reflect(&-light_dir, &N);
      tmp1 = tmp2[0]*dir[0] + tmp2[1]*dir[1] + tmp2[2]*dir[2];
      specular_light_intensity += light.intensity * mat.specular_exponent.powf(0.0f64.max(tmp1));
   }
   *rgb = mat.diffuse_color * diffuse_light_intensity * mat.albedo[0] + Vector3::new(1.,1.,1.) * specular_light_intensity * mat.albedo[1] + rgb_reflect * mat.albedo[2] + rgb_refract * mat.albedo[3];
}


fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>)
{
   const N_X: usize = 1024;
   const N_Y: usize = 800;

   // Initialize the image:
   let mut rgb_image: Vec<u8> = vec![0; 3 * N_X * N_Y];

   let fov = std::f64::consts::PI / 3.;
   let eye = Vector3::new(0., 0., 0.);
   
   // TODO parallelize
   // Iterate over each pixel (i, j) = (x, y) = (col, row):
   for j in 0..N_Y 
   {
      for i in 0..N_X 
      {
         let dir_x = (i as f64 + 0.5) - (N_X as f64 / 2.);
         let dir_y = -(j as f64 + 0.5) + (N_Y as f64 / 2.);
         let dir_z = -(N_Y as f64) / (2. * (fov / 2.).tan());
         let mut dir = Vector3::new(dir_x, dir_y, dir_z);
         dir.normalize_mut();

         let mut rgb = Vector3::new(0., 0., 0.);
         ray_cast(&mut rgb, &eye, &dir, spheres, lights, 0);
         
         // Set the color
         rgb_image[3 * (j * N_X + i) + 0] = rgb[0] as u8;
         rgb_image[3 * (j * N_X + i) + 1] = rgb[1] as u8;
         rgb_image[3 * (j * N_X + i) + 2] = rgb[2] as u8;
      }
   }

   write_ppm("rgb.ppm", &rgb_image, N_X, N_Y, 3);
}


fn write_ppm(path: &str, rgb_image: &Vec<u8>, n_x: usize, n_y: usize, nchannels: usize)
{
   let f = File::create(path).expect(format!("Error creating file {}", path).as_str());
   let mut stream = BufWriter::new(&f);
   let header = format!("P6\n{} {}\n255\n", n_x, n_y);
   stream.write(header.as_bytes()).expect("Problem writing header.");

   // Not making any assumptions on how the image data is stored
   for subpxl in 0..nchannels * n_x * n_y {
      stream.write(&[rgb_image[subpxl] as u8]).expect("Error writing pixels.");
   }

   stream.flush().expect("Error flushing buffer stream");
}




fn main() {
   println!("step2.rs");

   let ivory = Material::new(1., Vector4::new(0.6,0.3,0.1,0.0), Vector3::new(0.4,0.4,0.3), 50.);
   let glass = Material::new(1.5, Vector4::new(0.,0.5,0.1,0.8), Vector3::new(0.6,0.7,0.8), 125.);
   let red_rubber = Material::new(1., Vector4::new(0.9,0.1,0.,0.), Vector3::new(0.3,0.1,0.1), 10.);
   let mirror = Material::new(1., Vector4::new(0.,10.,0.8,0.), Vector3::new(1.,1.,1.), 1425.);
   
   let spheres = vec![
      Sphere::new(Vector3::new(-3., 0., -16.), 2., ivory),
      Sphere::new(Vector3::new(-1.0, -1.5, -12.), 2., glass),
      Sphere::new(Vector3::new(1.5, -0.5, -18.), 2., red_rubber),
      Sphere::new(Vector3::new(7., 5., -18.), 2., mirror),
   ];

   let lights = vec![
      Light::new(Vector3::new(-20.,20.,20.), 1.5),
      Light::new(Vector3::new(30.,50.,-25.), 1.8),
      Light::new(Vector3::new(30.,20.,30.), 1.7),
   ];

   render(&spheres, &lights);
}
