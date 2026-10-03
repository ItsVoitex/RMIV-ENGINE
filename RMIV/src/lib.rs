
mod managers;
pub mod application;
mod renderer;
pub mod templates;


pub mod prelude {
    pub use crate::application::app::{App};
    pub use crate::templates::object::Object;
    pub use crate::managers::{asset::AssetManager,input::Key,Action,MouseButton};
    pub use glam::{vec2,vec3,vec4};
}