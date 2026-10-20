



use rmiv_engine::prelude::*;

pub struct Player
{
    pub entity:Entity,
}

impl Player
{


    pub fn create(texture_id: u32,length:f32,width:f32,depth:f32, shader_id: u32,world:&mut World) -> Player
    {
        let entity = world.spawn((
            Transform
        {
            scale:vec3(length,width,depth),
            position:vec3(0.0, 0.0, 0.0),
            rotation:0.0
        },
        Render
        {
            texture_id:Some(texture_id),
            shader_id:shader_id,
            renderlayers:(1,1)
        },
        Hitbox{
            x:50.0,
            y:50.0,
            z:5.0,
        }
        ));
         
    
        Player
        {
            entity: entity,
        }
    }

    pub fn moved(&mut self,platform:&mut Platform,delta_time:&f32,window_handle:&WindowHandle,worldoffset:&mut Vec3)->bool
    {
        let moveleft = platform.Key_is_pressed(window_handle,KeyCode::A)  || platform.Key_is_pressed(window_handle,KeyCode::Left);
        let moveright = platform.Key_is_pressed(window_handle,KeyCode::D)  || platform.Key_is_pressed(window_handle,KeyCode::Right);
        let moveup = platform.Key_is_pressed(window_handle,KeyCode::W)  || platform.Key_is_pressed(window_handle,KeyCode::Up);
        let movedown = platform.Key_is_pressed(window_handle,KeyCode::S)  || platform.Key_is_pressed(window_handle,KeyCode::Down);
        let mut temp = false;
        if moveleft
        {
            worldoffset.x -= 480.0 * delta_time;  
            temp = true;
        }
        if moveright
        {
            worldoffset.x += 480.0 * delta_time; 
            temp = true; 
        }
        if moveup
        {
            worldoffset.y += 480.0 * delta_time;  
            temp = true;
        }
        if movedown
        {
            worldoffset.y -= 480.0 * delta_time;  
            temp = true;
        }
        return temp;
    }


}