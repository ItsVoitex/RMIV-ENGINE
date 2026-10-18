



use rmiv_engine::prelude::*;

pub struct Player
{
    pub entity:Entity,
    on_ground:bool,
    is_jumping:bool,
    velocity:Vec3
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
            on_ground: false,
            is_jumping:false, 
            velocity:vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn moved(&mut self,platform:&mut Platform,transform:&mut Transform,delta_time:&f32,window_handle:&WindowHandle) 
    {
        let jump = (platform.Key_is_pressed(window_handle,KeyCode::Space) || platform.Key_is_pressed(window_handle,KeyCode::Up) || platform.Key_is_pressed(window_handle,KeyCode::W)|| platform.mouse_is_pressed(window_handle,MouseButton::Button1))&& self.on_ground ;
        let moveleft = platform.Key_is_pressed(window_handle,KeyCode::A)  || platform.Key_is_pressed(window_handle,KeyCode::Left);
        let moveright = platform.Key_is_pressed(window_handle,KeyCode::D)  || platform.Key_is_pressed(window_handle,KeyCode::Right);
       
        
        if self.is_jumping
        {
            self.velocity.y += 15000.0 * delta_time;
            transform.position.y += self.velocity.y * delta_time;
            if transform.position.y >= -280.0
            {
                self.is_jumping = false;
                self.velocity.y = 0.0;
            }
        }
        if !self.on_ground && !self.is_jumping
        {
            self.velocity.y -= 7000.0 *delta_time;
            transform.position.y += self.velocity.y * delta_time;
            if transform.position.y <= -500.0 
            {
                transform.position.y = -500.0;
                self.velocity.y = 0.0;
                self.on_ground = true
            }
            
            
        }

        if moveleft
        {
            transform.position.x -= 480.0 * delta_time;
           
            
        }
        if moveright
        {
            transform.position.x += 480.0 * delta_time;
            
        }
        if jump
        {
            
            self.is_jumping = true;
            self.on_ground = false;

        }

        if transform.position.x + 38.4*1.5 < -921.6
        {
            transform.position.x = -921.6;   
            
        }
        if transform.position.x > 961.6
        {
            transform.position.x = 908.6;   
            
        }
    }


}