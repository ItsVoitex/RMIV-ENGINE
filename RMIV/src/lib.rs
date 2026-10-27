
pub mod managers;
pub mod application;
pub mod renderer;
pub mod templates;


pub mod prelude {
    pub use crate::application::{app::{App,}};
    pub use crate::managers::prelude::*;
    pub use crate::renderer::prelude::*;
    pub use glam::{vec2,vec3,vec4,Vec2,Vec3,Vec4};
    
}
pub mod backend{
    pub use crate::renderer::backend::*;
    pub use crate::managers::backend::*;
    pub use glam::{vec2,vec3,vec4,Vec2,Vec3,Vec4};
}
