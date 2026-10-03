use std::collections::HashMap;
use crate::renderer::{Mesh, Shader, Texture};

pub struct AssetManager
{
    pub textures:HashMap<u32,Texture>,
    pub shaders:HashMap<u32,Shader>,
    pub meshes:HashMap<u32,Mesh>,
    used_paths:HashMap<String,u32>,
    shader_index:u32,
    texture_index:u32,
    mesh_index:u32
}

impl AssetManager
{
    pub fn new() -> AssetManager
    {
        AssetManager 
        { 
            textures: HashMap::new(), 
            shaders: HashMap::new(), 
            meshes: HashMap::new(),
            used_paths:HashMap::new(),
            shader_index:0,
            texture_index:0,
            mesh_index:0
        }
    }
    //since this one is a little long pretty much it generates textures meshes or shaders returning an id
    //it also checks if you use the same file path to prevent duplicate files from being loaded
    pub fn load_textured_mesh(&mut self,texture_path:&str,width:f32,height:f32,depth:f32) -> (u32,u32)
    {
        (self.load_texture_from_file(texture_path),self.load_mesh(width, height, depth))
    }
    pub fn load_texture_from_file(&mut self,texture_path:&str) -> u32
    {
        if let Some(_id) = self.used_paths.get(texture_path)
        {
                println!("this texture has already been loaded");
                return *_id;
        }
        
        self.textures.insert(self.texture_index, Texture::create(texture_path));
        self.used_paths.insert(texture_path.to_string(),self.texture_index);
        let temp = self.texture_index;
        self.texture_index += 1;
        return temp;   
    }
    pub fn get_texture_id(&self,texture_path:&str) -> u32
    {
        if let Some(_id) = self.used_paths.get(texture_path)
        {
            return *_id;
        }
        else {
            panic!("failed to get texture with path: {}",texture_path);
        }
    }

    pub fn load_shader_from_file(&mut self,shader_path:&[&str;2]) -> u32
    {
        if let Some(_id) = self.used_paths.get(shader_path[0])
        {
            return *_id;
        }
        self.shaders.insert(self.shader_index, Shader::new(shader_path[0],shader_path[1]));
        self.used_paths.insert( shader_path[0].to_string(),self.shader_index);
        let temp = self.shader_index;
        self.shader_index += 1;
        return temp;
    }
    pub fn get_shader_id(&self,shader_path:&[&str;2]) ->u32
    {
        if let Some(_id) = self.used_paths.get(shader_path[0])
        {
            return *_id;
        }
        else {
            panic!("failed to get shader with path: {} and {}",shader_path[0],shader_path[1]);
        }
    }


    pub fn load_default_shader(&mut self) -> u32
    {
        self.shaders.insert(self.shader_index, Shader::create());
        self.used_paths.insert("default-shader".to_string(), self.shader_index);
        let temp = self.shader_index;
        self.shader_index += 1;
        return temp;
    }
    
    pub fn load_mesh(&mut self,width:f32,height:f32,depth:f32) -> u32
    {
        
        let vertices: [f32; 20] = [
            width / 2.0,   height / 2.0,    depth,  0.0, 0.0,
           -width / 2.0,   height / 2.0,    depth,  1.0, 0.0,
           -width / 2.0,  -height / 2.0,    depth,  1.0, 1.0,
            width / 2.0,  -height / 2.0,    depth,  0.0, 1.0,
        ];

        let indices: [u32; 6]= 
        [
            0,  1,  2,
            0,  2,  3,
        ];
        self.meshes.insert(self.mesh_index, Mesh::gen_rectangle(vertices,indices));
        let temp = self.mesh_index;
        self.mesh_index += 1;
        return temp;
    }
    


}



