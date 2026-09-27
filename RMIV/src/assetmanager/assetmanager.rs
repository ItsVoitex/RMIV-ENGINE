use std::collections::HashMap;

use crate::renderer::{Mesh, Shader, Texture};

pub struct AssetManager
{
    pub textures:HashMap<String,Texture>,
    pub shaders:HashMap<String,Shader>,
    pub meshes:HashMap<String,Mesh>,

    used_paths:HashMap<String,String>
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
        }
    }
    pub fn load_all(&mut self,texture_path:&str,width:f32,height:f32,depth:f32,id:&str) -> (String,String)
    {
        self.load_texture_from_file(texture_path, id);
        self.load_mesh(width, height, depth, id);
        
        return (id.to_string(), self.load_shader().to_string())
    }
    pub fn load_textured_mesh(&mut self,texture_path:&str,width:f32,height:f32,depth:f32,id:&str) -> String
    {
        self.load_texture_from_file(texture_path, id);
        self.load_mesh(width, height, depth, id);
        
        return id.to_string()
    }
    pub fn load_all_from_file(&mut self,texture_path:&str,shader_path:&[&str;2],width:f32,height:f32,depth:f32,id:&str) -> String
    {
        self.load_texture_from_file(texture_path, id);
        self.load_mesh(width, height, depth, id);
        self.load_shader_from_file(shader_path,id);
        return id.to_string();
    }
    pub fn load_texture_from_file(&mut self,texture_path:&str,texture_id:&str) -> String
    {
        if let Some(_texture) = self.textures.get(texture_id)
        {
            println!("texture with that id already exits");
            return texture_id.to_string();
        }
        if let Some(id) = self.used_paths.get(texture_path)
        {
                println!("this texture has already been loaded");
                return id.to_string();
        }
        self.textures.insert(texture_id.to_string(), Texture::create(texture_path));
        self.used_paths.insert(texture_path.to_string(),texture_id.to_string());
        return texture_id.to_string();
        
    }

    pub fn load_shader_from_file(&mut self,shader_path:&[&str;2],shader_id:&str) -> String
    {
        if let Some(_shader) = self.shaders.get(shader_id)
        {
            println!("shader with that id already exits");
            return shader_id.to_string();
        }
        if let Some(id) = self.used_paths.get(shader_path[0])
        {
                println!("this shader has already been loaded");
                return id.to_string();
        }
        self.shaders.insert(shader_id.to_string(), Shader::new(shader_path[0],shader_path[1]));
        self.used_paths.insert( shader_path[0].to_string(),shader_id.to_string());
        return shader_id.to_string();
    }


    pub fn load_shader(&mut self) -> String
    {
        if let Some(_shader) = self.shaders.get("default")
        {
            println!("shader with that id already exits");
            return "default".to_string();
        }
        if let Some(id) = self.used_paths.get("default-shader")
        {
                println!("this shader has already been loaded");
                return id.to_string();
        }
        self.shaders.insert("default".to_string(), Shader::create());
        self.used_paths.insert("default-shader".to_string(), "default".to_string());
        return "default".to_string();
    }

    pub fn load_mesh(&mut self,width:f32,height:f32,depth:f32,mesh_id:&str) -> String
    {
        if let Some(_mesh) = self.meshes.get(mesh_id)
        {
            println!("mesh with that id already exits");
            return mesh_id.to_string();
        }
        let vertices: [f32; 20] = 
        [
            960.0+width/2.0,  540.9+height/2.0,  depth, 0.0, 0.0,
            960.0-width/2.0,  540.9+height/2.0,  depth, 1.0, 0.0,
            960.0-width/2.0,  540.9-height/2.0,  depth, 1.0, 1.0,
            960.0+width/2.0,  540.9-height/2.0,  depth, 0.0, 1.0,
        ];
        let indices: [u32; 6]= 
        [
            0,  1,  2,
            0,  2,  3,
        ];
        self.meshes.insert(mesh_id.to_string(), Mesh::gendata(vertices,indices));
        return mesh_id.to_string();
    }


}