
mod managers;
pub mod application;
pub mod renderer;
pub mod templates;


pub mod prelude {
    pub use crate::application::{app::{App,SystemType::{Startup,Update}}};
    pub use crate::templates;
    pub use crate::managers::{platform::Platform,window::WindowHandle,asset::AssetManager,world::{World,Entity,Transform,Render,Hitbox,Camera},input::{KeyCode::*,KeyCode},Action,MouseButton};
    pub use glam::{vec2,vec3,vec4,Vec2,Vec3,Vec4};
    
}