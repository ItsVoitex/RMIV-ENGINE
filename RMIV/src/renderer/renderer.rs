use std::ffi::CString;


use crate::{Object, assetmanager::assetmanager::AssetManager};



pub struct Renderer
{
    texture_name:CString,
    offset_name:CString
}


impl Renderer
{
    pub fn new()-> Renderer
    {
        Renderer{
            texture_name:CString::new("texture1").unwrap(),
            offset_name:CString::new("offset").unwrap()
        }
        
    }

    pub fn draw_object(&mut self,object:&mut Object,assetmanager:&AssetManager)
    {
        let shader = assetmanager.shaders.get(&object.shader_id).unwrap();
        let mesh = assetmanager.meshes.get(&object.mesh_id).unwrap();
        let texture = assetmanager.textures.get(&object.texture_id).unwrap();
        unsafe {
            let offset_location = gl::GetUniformLocation(shader.id,self.offset_name.as_ptr());
            let texture_location = gl::GetUniformLocation(shader.id,self.texture_name.as_ptr());
            gl::Uniform1i(texture_location,0);

            shader.use_program();
            mesh.bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::Uniform3f(offset_location, object.position.x,object.position.y,object.position.z);
            gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
        }
    }
}