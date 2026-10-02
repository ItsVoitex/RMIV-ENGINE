



use rmiv_engine::prelude::*;

pub struct Player
{
    pub object:Object,
    on_ground:bool,
    is_jumping:bool,
}

impl Player
{


    pub fn create(texture_id: u32,length:f32,width:f32,depth:f32, shader_id: u32) -> Player
    {
        let object = Object::create(texture_id, length,width,depth, shader_id);

        
        Player
        {
            object: object,
            on_ground: false,
            is_jumping:false, 
        }
    }

    pub fn moved<T>(&mut self,app:&mut App<T>,delta_time:&f32,window_id:u32) 
    {
       
        if self.is_jumping
        {
            self.object.velocity -= 15000.0 * delta_time;
            self.object.position.y += self.object.velocity.y * delta_time;
            if self.object.position.y <= 280.0
            {
                self.is_jumping = false;
                self.object.velocity.y = 0.0;
            }
        }
        if !self.on_ground && !self.is_jumping
        {
            self.object.velocity.y += 7000.0 *delta_time;
            self.object.position.y += self.object.velocity.y * delta_time;

            if self.object.position.y >= 500.0 
            {
                self.object.position.y = 500.0;
                self.object.velocity.y = 0.0;
                self.on_ground = true
            }
            
            
        }
        if app.key_is_pressed(window_id,Key::T)
        {
            self.object.position.y += 270.0 * delta_time;
        }
        if app.key_is_pressed(window_id,Key::A)  || app.key_is_pressed(window_id,Key::Left)
        {
            self.object.position.x -= 480.0 * delta_time;
           
            
        }
        if app.key_is_pressed(window_id,Key::D)  || app.key_is_pressed(window_id,Key::Right) 
        {
            self.object.position.x += 480.0 * delta_time;
            
        }
        if (app.key_is_pressed(window_id,Key::Space) || app.key_is_pressed(window_id,Key::Up) || app.key_is_pressed(window_id,Key::W)|| app.mouse_is_pressed(window_id,MouseButton::Button1))&& self.on_ground 
        {
            
            self.is_jumping = true;
            self.on_ground = false;

        }
        if self.object.position.x + 38.4*1.5 < -921.6
        {
            self.object.position.x = -921.6;
            
            
        }
    }


}