use std::ffi::CString;
use crate::swapper::TextureSwapper;
use crate::renderer::texture::Texture;
use crate::renderer::shader;
use gl;
use crate::renderer::mesh::Mesh;



pub struct Object
{
    pub x:f32,
    pub y:f32,
    z:f32,
    pub velocity_x:f32,
    pub velocity_y:f32,
    offset:i32,
    pub object_data:ObjectData
}

pub struct ObjectData
{
    pub current_texture:u32,
    pub textures:TextureSwapper,
    pub shader:shader::Shader,
    pub object_type:String,
    data:Mesh,
}


impl Object 
{
    pub fn multi_create(object_data:&ObjectData,object_type:String) -> Object
    {
        
        let offset_location;
        unsafe {
            let offset_name = CString::new("offset").unwrap();

            offset_location = gl::GetUniformLocation(object_data.shader.id,offset_name.as_ptr()
        );}

        let mut temp = Vec::new();
        for i in 0..object_data.textures.textures.len()
        {
            temp.push(Texture{id: object_data.textures.textures[i].id});
        }
        
        Object
        {
            x:0.0,
            y:0.0,
            z:0.0,
            velocity_x:0.0,
            velocity_y:0.0,
            offset:offset_location,
            object_data : ObjectData {
                current_texture: object_data.current_texture,
                textures: TextureSwapper { textures: temp },
                shader: shader::Shader { id: object_data.shader.id },
                object_type: object_type, 
                data: Mesh{vbo: object_data.data.vbo ,vao:object_data.data.vao , ebo: object_data.data.ebo},
                }
    
        }
    }
    pub fn create(width:f32,height:f32,depth:f32,texture_path:&[&str]) -> Object
    {
        
        
        let shader = shader::Shader::create();
        let offset_location;

        unsafe {
            let offset_name = CString::new("offset").unwrap();

            offset_location = gl::GetUniformLocation(shader.id,offset_name.as_ptr()
        );}
        
        Object
        {
            x:0.0,
            y:0.0,
            z:0.0,
            velocity_x:0.0,
            velocity_y:0.0,
            offset:offset_location,
            object_data : ObjectData::create(width,height,depth, texture_path,String::from("object"))
        }
    }
    
    
    pub fn draw(&mut self)
    {
        if self.object_data.current_texture == 0
        {
            self.object_data.current_texture = self.object_data.textures.textures[0].id;
        }

        let texture_location;
        unsafe { 
            let texture_name = CString::new("texture1").unwrap();
            texture_location = gl::GetUniformLocation(self.object_data.shader.id,texture_name.as_ptr());
            gl::Uniform1i(texture_location,0);

            self.object_data.shader.use_program();
            self.object_data.data.mesh_bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.object_data.current_texture);
            gl::Uniform3f(self.offset, self.x, self.y,self.z);
            gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
        }
    }
   

   
   
}

impl ObjectData
{
    pub fn create(width:f32,height:f32,depth:f32,texture_path:&[&str],object_type:String) -> ObjectData
    {
        let object_vertices: [f32; 20] = [
        960.0+width/2.0,  540.9+height/2.0,  depth, 0.0, 0.0,
        960.0-width/2.0,  540.9+height/2.0,  depth, 1.0, 0.0,
        960.0-width/2.0,  540.9-height/2.0,  depth, 1.0, 1.0,
        960.0+width/2.0,  540.9-height/2.0,  depth, 0.0, 1.0,
        ];
        let indices: [u32; 6]= [
        0,  1,  2,
        0,  2,  3,
        ];
        let shader = shader::Shader::create();
        ObjectData {
                current_texture: 0,
                textures: TextureSwapper::create(texture_path),
                shader: shader,
                data: Mesh::gendata(object_vertices,indices),
                object_type: object_type,

        
        }
    }
    
    
    
}