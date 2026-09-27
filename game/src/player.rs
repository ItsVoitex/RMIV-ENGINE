use std::ffi::CString;


use rmiv_engine::Object;
use rmiv_engine::application;
use rmiv_engine;
use rmiv_engine::{Key,Action,App};
pub struct Player
{
    pub object:Object,
    on_ground:bool,
}

impl Player
{


    pub fn create(width: f32, height: f32, depth: f32, texture_path: &[&str]) -> Player
    {
        let object = Object::create(width, height, depth, texture_path);

        
        Player
        {
            object: object,
            on_ground: false
        }
    }
    pub fn draw(&mut self) {
        self.object.draw();
        
    }

    pub fn moved(&mut self,app:&mut App,delta_time:&f32,window_id:&str) 
    {
       
        
        if !self.on_ground
        {
            self.object.velocity_y += 50.0;
            self.object.y += self.object.velocity_y * delta_time;

            if self.object.y >= 504.9
            {
                self.object.y = 504.9;
                self.object.velocity_y = 0.0;
                self.on_ground = true
            }
            
            
        }
        if app.key_is_pressed(window_id,Key::T)
        {
            self.object.y += 270.0 * delta_time;
        }
        if app.key_is_pressed(window_id,Key::A)  || app.key_is_pressed(window_id,Key::Left)
        {
            self.object.x -= 480.0 * delta_time;
           
            
        }
        if app.key_is_pressed(window_id,Key::D)  || app.key_is_pressed(window_id,Key::Right) 
        {
            self.object.x += 480.0 * delta_time;
            
        }
        if app.key_is_pressed(window_id,Key::Space) && self.on_ground
        {
            self.object.y -= (270.0*130.0) * delta_time;
            self.on_ground = false;

        }
        if self.object.x + 38.4*1.5 < -921.6
        {
            self.object.x = -921.6;
            
            
        }
    }


}