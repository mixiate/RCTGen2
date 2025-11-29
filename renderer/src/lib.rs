pub mod model;
pub mod palette;
mod raytrace;
mod renderer;

pub use embree::Device;
pub use raytrace::Scene;
pub use renderer::{Framebuffer, IndexedImage, Light, render_scene};
