
mod managers;
pub mod application;
mod renderer;
pub mod object;


pub mod prelude {
    pub use crate::application::app::{App};
    pub use crate::{object::Object};
    pub use crate::managers::{asset::AssetManager,input::Key,Action,MouseButton};
}