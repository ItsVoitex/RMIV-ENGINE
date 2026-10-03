
use crate::{application::App};
use glam::Vec3;

use crate::prelude::*;



pub struct Object
{
    pub position:Vec3,
    pub scale:Vec3,
    pub velocity:Vec3,
    pub hitbox:Option<Vec3>,
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
            hitbox:None,
            texture_id: texture_id,
            shader_id:shader_id,
        }
    }
    pub fn set_hitbox(&mut self,length:f32,width:f32,depth:f32)
    {
        self.hitbox = Some(glam::vec3(length, width,depth));
    }
    pub fn set_texture_id(&mut self,texture_id:u32)
    {
        self.texture_id = texture_id;
    }
    pub fn set_shader_id(&mut self,shader_id:u32)
    {
        self.shader_id = shader_id;
    }
    pub fn check_collision(&mut self,objects:&Vec<Object>,skip_objects_with_no_hitbox:bool) ->bool
    {
        if self.hitbox != None
        {
            let tr = vec3( 0.5*self.hitbox.unwrap().x + self.position.x,   0.5*self.hitbox.unwrap().y + self.position.y,    0.5*self.hitbox.unwrap().z + self.position.z);
            let bl = vec3(-0.5*self.hitbox.unwrap().x + self.position.x,  -0.5*self.hitbox.unwrap().y + self.position.y,    0.5*self.hitbox.unwrap().z + self.position.z);
            for i in 0..objects.len(){
                if objects[i].hitbox != None
                {
                    let tr1 = vec3( 0.5*objects[i].hitbox.unwrap().x + objects[i].position.x,   0.5*objects[i].hitbox.unwrap().y + objects[i].position.y,    0.5*objects[i].hitbox.unwrap().z + objects[i].position.z);
                    let bl1 = vec3(-0.5*objects[i].hitbox.unwrap().x + objects[i].position.x,  -0.5*objects[i].hitbox.unwrap().y + objects[i].position.y,    0.5*objects[i].hitbox.unwrap().z + objects[i].position.z);
                    if (bl.x <= tr1.x && bl1.x <= tr.x) &&  (bl.y <= tr1.y && bl1.y <= tr.y) //&& (bl.z <= tr1.z && bl1.z <= tr.z)
                    {
                        return true;
                    }
                    continue;
                }
                if skip_objects_with_no_hitbox{
                    continue;
                }
                else {
                    panic!("comparison object to check collisions has no hitbox compononent and skipping objects with no hitbox is not enabled")
                } 
            }
            return false;
        }
        else {
            panic!("first object has no hitbox component failure to check collision");
        }

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
/*two object collision

let tr = vec3( 0.5*objects[i].hitbox.unwrap().x + objects[i].position.x,   0.5*objects[i].hitbox.unwrap().y + objects[i].position.y,    0.5*objects[i].hitbox.unwrap().z + objects[i].position.z);
let bl = vec3(-0.5*objects[i].hitbox.unwrap().x + objects[i].position.x,  -0.5*objects[i].hitbox.unwrap().y + objects[i].position.y,    0.5*objects[i].hitbox.unwrap().z + objects[i].position.z);
let tr1 = vec3( 0.5*objects[i+1].hitbox.unwrap().x + objects[i+1].position.x,   0.5*objects[i+1].hitbox.unwrap().y + objects[i+1].position.y,    0.5*objects[i+1].hitbox.unwrap().z + objects[i+1].position.z);
let bl1 = vec3(-0.5*objects[i+1].hitbox.unwrap().x + objects[i+1].position.x,  -0.5*objects[i+1].hitbox.unwrap().y + objects[i+1].position.y,    0.5*objects[i+1].hitbox.unwrap().z + objects[i+1].position.z);
     
if (bl.x <= tr1.x && bl1.x <= tr.x) &&  (bl.y <= tr1.y && bl.y <= tr.y) //&& (bl.z <= tr.z && bl1.z <= tr1.z)
{
    print!("colliding")
}
*/
/* source
pub fn check_collision(self:&Object,objects:&Vec<Object>,skip_objects_with_no_hitbox:bool) ->bool
{
    if self.hitbox != None
    {
        let tr = vec3( 0.5*self.hitbox.unwrap().x + self.position.x,   0.5*self.hitbox.unwrap().y + self.position.y,    0.5*self.hitbox.unwrap().z + self.position.z);
        let bl = vec3(-0.5*self.hitbox.unwrap().x + self.position.x,  -0.5*self.hitbox.unwrap().y + self.position.y,    0.5*self.hitbox.unwrap().z + self.position.z);
        for i in 0..objects.len(){
            if objects[i].hitbox != None
            {
                let tr1 = vec3( 0.5*objects[i].hitbox.unwrap().x + objects[i].position.x,   0.5*objects[i].hitbox.unwrap().y + objects[i].position.y,    0.5*objects[i].hitbox.unwrap().z + objects[i].position.z);
                let bl1 = vec3(-0.5*objects[i].hitbox.unwrap().x + objects[i].position.x,  -0.5*objects[i].hitbox.unwrap().y + objects[i].position.y,    0.5*objects[i].hitbox.unwrap().z + objects[i].position.z);
                if (bl.x <= tr1.x && bl1.x <= tr.x) &&  (bl.y <= tr1.y && bl1.y <= tr.y) //&& (bl.z <= tr1.z && bl1.z <= tr.z)
                {
                    return true;
                }
                continue;
            }
            if skip_objects_with_no_hitbox{
                continue;
            }
            else {
                panic!("comparison object to check collisions has no hitbox compononent and skipping objects with no hitbox is not enabled")
            } 
        }
        return false;
    }
    else {
        panic!("first object has no hitbox component failure to check collision");
    }

}
*/