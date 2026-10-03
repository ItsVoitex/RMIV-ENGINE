use std::ffi::CString;


use crate::{renderer::Mesh, templates::object::Object, managers::asset::AssetManager};



pub struct Renderer
{
    texture_name:CString,
    offset_name:CString,
    scale_name:CString,
    mesh:Mesh,
    notextureid:u32
    
}


impl Renderer
{
    pub fn new()-> Renderer
    {
        //binds default mesh to be used and sent to the gpu
         let vertices: [f32; 20] = [
            0.5,   -0.5,    0.5,  0.0, 0.0,
           -0.5,   -0.5,    0.5,  1.0, 0.0,
           -0.5,  0.5,    0.5,  1.0, 1.0,
            0.5,  0.5,    0.5,  0.0, 1.0,
        ];
        let indices: [u32; 6]= 
        [
            0,  1,  2,
            0,  2,  3,
        ];
        Renderer{
            texture_name:CString::new("texture1").unwrap(),
            offset_name:CString::new("offset").unwrap(),
            scale_name:CString::new("scale").unwrap(),
            mesh:Mesh::gen_rectangle(vertices, indices),
            notextureid:0
        }
        
    }

    pub fn draw_object(&mut self,object:&mut Object,assetmanager:&mut AssetManager)
    {
        //when there is no texture kind of obvious need to implement an alternative instead of a file
        if self.notextureid == 0
        {
            self.notextureid = assetmanager.load_texture_from_file("Assets/textures/NoTexture.png");
        }
        let shader = assetmanager.shaders.get(&object.shader_id);
        let shader = match shader{
            Some(sh) => sh,
            _ => panic!("failed to get shader")
        };
        let mesh = &self.mesh;
        let texture = assetmanager.textures.get(&object.texture_id);
        
        
        let texture = match texture{
            Some(sh) => sh,

            _ => {let temp = assetmanager.textures.get(&self.notextureid).unwrap();temp}
        };
        
        unsafe {
            //binds textures shaders meshes and send the object data to the gpu
            shader.use_program();
            mesh.bind();
            let offset_location = gl::GetUniformLocation(shader.id,self.offset_name.as_ptr());
            let scale_location = gl::GetUniformLocation(shader.id,self.scale_name.as_ptr());
            let texture_location = gl::GetUniformLocation(shader.id,self.texture_name.as_ptr());
            gl::Uniform1i(texture_location,0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::Uniform3f(offset_location, object.position.x,object.position.y,object.position.z);
            gl::Uniform3f(scale_location, object.scale.x,object.scale.y,object.scale.z);
            gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
        }
    }
    pub fn draw_objects(&mut self,assets:&mut AssetManager,objects:Vec<&Object>)
    {
        //draws multiple objects instead of one
        for object in objects.iter()
        {
            if self.notextureid == 0
            {
                self.notextureid = assets.load_texture_from_file("Assets/textures/NoTexture.png");
            }
            let shader = assets.shaders.get(&object.shader_id);
            let shader = match shader{
                Some(sh) => sh,
                _ => panic!("failed to get shader")
            };
            let mesh = &self.mesh;
            let texture = assets.textures.get(&object.texture_id);
        
        
            let texture = match texture{
                Some(sh) => sh,

                _ => {let temp = assets.textures.get(&self.notextureid).unwrap();temp}
            };
        
            unsafe {
                shader.use_program();
                mesh.bind();
                let offset_location = gl::GetUniformLocation(shader.id,self.offset_name.as_ptr());
                let scale_location = gl::GetUniformLocation(shader.id,self.scale_name.as_ptr());
                let texture_location = gl::GetUniformLocation(shader.id,self.texture_name.as_ptr());
                gl::Uniform1i(texture_location,0);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
                gl::Uniform3f(offset_location, object.position.x,object.position.y,object.position.z);
                gl::Uniform3f(scale_location, object.scale.x,object.scale.y,object.scale.z);
                gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
            }
        }
        
    }
}