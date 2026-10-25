pub mod asset;
pub mod input;
pub mod audio;
pub mod world;
pub mod window;
pub mod platform;
pub mod commands;

pub mod prelude{
    pub use crate::managers::{
        platform::Platform,window::{WindowHandle,Window},
        asset::{AssetManager,Texture,CustomShader,AssetHandle},
        world::{World,Entity,Transform,Render,Hitbox,Camera},
        input::{KeyCode::*,KeyCode,Action,MouseButton}};
}
pub mod backend {
    pub use crate::managers::{
        platform::Platform,window::{WindowHandle,Window},
        asset::{AssetManager,Texture,CustomShader,AssetHandle},
        world::{World,Entity,Transform,Render,Hitbox,Camera},
        input::{KeyCode::*,KeyCode,Action,MouseButton}};
}