pub type Ray = ray::Ray<f32>;
mod interaction;
pub mod ray;
mod surface_interaction;

pub use interaction::Interaction;
pub use surface_interaction::SurfaceInteraction;
