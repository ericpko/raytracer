/**
 * This is just a module called geometry and the files inside
 * the geometry directory are also modules, but submodules of this
 * module.
 * 
 * This is the same thing as creating a mod.rs file inside of the 
 * geometry directory (similar to scene/mod.rs). This method is the 
 * now preferred (Rust 2018) way of creating modules so you don't end
 * up with a bunch of mod.rs files.
 */
pub mod sphere;
pub use self::sphere::Sphere;
