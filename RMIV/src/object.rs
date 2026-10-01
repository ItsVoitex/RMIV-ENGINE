
use crate::{App};
use glam::Vec3;





pub struct Object
{
    pub position:Vec3,
    pub scale:Vec3,
    pub velocity:Vec3,
    pub texture_id:u32,
    pub shader_id:u32,
}



impl Object 
{
    pub fn create(texture_id:u32,length:f32,width:f32,depth:f32,shader_id:u32) -> Object
    {
        
        Object
        {
            position:glam::vec3(0.0, 0.0, 0.0),
            scale:glam::vec3(length, width, depth),
            velocity:glam::vec3(0.0, 0.0,0.0),
            texture_id: texture_id,
            shader_id:shader_id,
        }
    }
    pub fn set_texture_id(&mut self,texture_id:u32)
    {
        self.texture_id = texture_id;
    }
    pub fn set_shader_id(&mut self,shader_id:u32)
    {
        self.shader_id = shader_id;
    }
    
    
    pub fn draw(&mut self,app:&mut App,window_id:u32)
    {
        if let Some(renderer) = app.renderer.get_mut(&window_id)
        {
            renderer.draw_object(self,&mut app.assets);
        }
        else {
            panic!("failed to get window with id: {}",window_id);
        } 
        
    }
   

   
   
}
