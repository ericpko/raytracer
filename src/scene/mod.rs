/**
 * This is just the module for scene. <scene> is a module. 
 * In rust, pretty much every .rs file is a module.
 * https://stackoverflow.com/questions/22596920/split-a-module-across-several-files
 * 
 * http://www.sheshbabu.com/posts/rust-module-system/
 */

pub mod camera;
pub use self::camera::Camera;

pub mod ray;
pub use self::ray::Ray;
