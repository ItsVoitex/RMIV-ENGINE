
use crate::App;
use glam::Vec2;
use glam::Vec3;




pub struct Object
{
    pub position:Vec3,
    pub velocity:Vec2,
    pub texture_id:String,
    pub mesh_id:String,
    pub shader_id:String,
}



impl Object 
{
    pub fn create(texture_id:&str,mesh_id:&str,shader_id:&str) -> Object
    {
        
        Object
        {
            position:glam::vec3(0.0, 0.0, 0.0),
            velocity:glam::vec2(0.0, 0.0),
            texture_id: texture_id.to_string(),
            mesh_id: mesh_id.to_string(),
            shader_id:shader_id.to_string(),
        }
    }
    
    
    pub fn draw(&mut self,app:&mut App)
    {
        app.renderer.draw_object(self,&mut app.assets);
    }
   

   
   
}
