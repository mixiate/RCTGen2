mod framebuffer;
pub mod image;
pub mod model;
pub mod palette;
mod raytrace;
mod renderer;

pub use embree::Device;
pub use framebuffer::Framebuffer;
pub use raytrace::Scene;
pub use renderer::{Light, render_scene};
