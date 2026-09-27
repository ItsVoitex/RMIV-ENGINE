
use rmiv_engine::Object;

use rmiv_engine;
use rmiv_engine::{Key,App};
pub struct Player
{
    pub object:Object,
    on_ground:bool,
}

impl Player
{


    pub fn create(texture_id: &str, mesh_id: &str, shader_id: &str) -> Player
    {
        let object = Object::create(texture_id, mesh_id, shader_id);

        
        Player
        {
            object: object,
            on_ground: false
        }
    }
    pub fn draw(&mut self,app:&mut App) {
        self.object.draw(app);
        
    }

    pub fn moved(&mut self,app:&mut App,delta_time:&f32,window_id:&str) 
    {
       
        if !self.on_ground
        {
            self.object.velocity.y += 50.0;
            self.object.position.y += self.object.velocity.y * delta_time;

            if self.object.position.y >= 504.9
            {
                self.object.position.y = 504.9;
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
        if app.key_is_pressed(window_id,Key::Space) && self.on_ground
        {
            self.object.position.y -= (270.0*130.0) * delta_time;
            self.on_ground = false;

        }
        if self.object.position.x + 38.4*1.5 < -921.6
        {
            self.object.position.x = -921.6;
            
            
        }
    }


}