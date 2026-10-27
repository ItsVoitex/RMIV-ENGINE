pub mod mesh;
pub mod shader;
pub mod texture;
pub mod renderer;


pub mod prelude {
    pub use crate::renderer::mesh::Mesh;
}
pub mod backend {
    pub use crate::renderer::{mesh::{MeshInstance,Mesh},shader::ShaderInstance,texture::TextureInstance,renderer::Renderer};

}