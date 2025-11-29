pub mod image;
pub mod model;
pub mod palette;
mod raytrace;
mod renderer;

pub use embree::Device;
pub use raytrace::Scene;
pub use renderer::{Framebuffer, Light, render_scene};
