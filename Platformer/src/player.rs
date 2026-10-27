



use rmiv_engine::prelude::*;

pub struct Player
{
    pub entity:Entity,
    on_ground:bool,
    is_jumping:bool,
    velocity:Vec3,
    pub rotationstate:Vec3
}

impl Player
{


    pub fn create(texture_id: &AssetHandle,length:f32,width:f32,depth:f32,world:&mut World,defaultmesh:&AssetHandle) -> Player
    {
        let entity = world.spawn((
            Transform
        {
            scale:vec3(length,width,depth),
            position:vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0,0.0,0.0), 
        },
        Render
        {
            texture_id:Some(*texture_id),
            shader_id:None,
            mesh_id:*defaultmesh,
            renderlayers:(1,1)
        },
        Hitbox{
            x:50.0,
            y:50.0,
            z:50.0
        }
        ));
         
      
        
        Player
        {
            entity: entity,
            on_ground: false,
            is_jumping:false, 
            velocity:vec3(0.0, 0.0, 0.0),
            rotationstate: vec3(0.0,0.0,0.0), 
        }
    }

    pub fn moved(&mut self,platform:&mut Platform,transform:&mut Transform,delta_time:&f32,window_handle:&WindowHandle) 
    {
        let jump = (platform.Key_is_pressed(window_handle,KeyCode::Space) || platform.Key_is_pressed(window_handle,KeyCode::Up) || platform.Key_is_pressed(window_handle,KeyCode::W)|| platform.mouse_is_pressed(window_handle,MouseButton::Button1))&& self.on_ground ;
        let moveleft = platform.Key_is_pressed(window_handle,KeyCode::A)  || platform.Key_is_pressed(window_handle,KeyCode::Left);
        let moveright = platform.Key_is_pressed(window_handle,KeyCode::D)  || platform.Key_is_pressed(window_handle,KeyCode::Right);
       
        
        if self.is_jumping
        {
            transform.rotation.x += 45.0*15.0 *delta_time;
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
            
            transform.rotation.x += 45.0*15.0 *delta_time;
            self.velocity.y -= 7000.0 *delta_time;
            transform.position.y += self.velocity.y * delta_time;
            if transform.position.y <= -500.0 
            {
                transform.rotation.x = self.rotationstate.x;
                self.rotationstate += 90.0;
                
                transform.position.y = -500.0;
                self.velocity.y = 0.0;
                self.on_ground = true
            } 
        }
        if transform.rotation.x > self.rotationstate.x && self.on_ground
        {
            transform.rotation.x -= 45.0*15.0*delta_time;
        }
        

        if moveleft && !(transform.position.x + 38.4*1.5/2.0 < -921.6)
        {
            transform.position.x -= 480.0 * delta_time;
           
            
        }
        if moveright && !(transform.position.x + 38.4*1.5/2.0 > 961.6)
        {
            transform.position.x += 480.0 * delta_time;
            
        }
        if jump
        {
            
            self.is_jumping = true;
            self.on_ground = false;

        }

        
    }


}