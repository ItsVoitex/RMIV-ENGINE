



use rmiv_engine::prelude::*;

pub struct Player
{
    pub entity:Entity,
    on_ground:bool,
    is_jumping:bool,
    velocity:Vec3,
    floor:f32,
    pub rotationstate:Vec3
}

impl Player
{


    pub fn create(texture_id: &AssetHandle,length:f32,height:f32,width:f32,world:&mut World,defaultmesh:&AssetHandle,floor:f32) -> Player
    {
        let entity = world.spawn((
            Transform
        {
            scale:vec3(length,height,width),
            position:vec3(-1.25, floor, -0.6),
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
            length:length-0.1,
            width:width-0.1,
            height:height-0.1
        }
        ));
         
      
        
        Player
        {
            entity: entity,
            on_ground: false,
            is_jumping:false, 
            velocity:vec3(0.0, 0.0, 0.0),
            rotationstate: vec3(0.0,0.0,0.0), 
            floor
        }
    }

    pub fn moved(&mut self,platform:&mut Platform,transform:&mut Transform,delta_time:&f32,window_handle:&WindowHandle) 
    {
        let jump = (platform.Key_is_pressed(window_handle,KeyCode::Space) || platform.Key_is_pressed(window_handle,KeyCode::Up) || platform.Key_is_pressed(window_handle,KeyCode::W)|| platform.mouse_is_pressed(window_handle,MouseButton::Button1))&& self.on_ground ;
        let moveleft = platform.Key_is_pressed(window_handle,KeyCode::A)  || platform.Key_is_pressed(window_handle,KeyCode::Left);
        let moveright = platform.Key_is_pressed(window_handle,KeyCode::D)  || platform.Key_is_pressed(window_handle,KeyCode::Right);
       
        
        if self.is_jumping
        {
            transform.rotation.z += 45.0*15.0 *delta_time;
            self.velocity.y += 46.0 * delta_time;
            transform.position.y += self.velocity.y * delta_time;
            if transform.position.y >= -0.5
            {
                self.is_jumping = false;
                self.velocity.y = 0.0;
            }
        }
        if !self.on_ground && !self.is_jumping
        {
            
            transform.rotation.z += 45.0*15.0 *delta_time;
            self.velocity.y -= 21.0 *delta_time;
            transform.position.y += self.velocity.y * delta_time;
            if transform.position.y <= self.floor
            {
                transform.rotation.z = self.rotationstate.z;
                self.rotationstate.z += 90.0;
                
                transform.position.y = self.floor;
                self.velocity.y = 0.0;
                self.on_ground = true
            } 
        }
        if transform.rotation.z > self.rotationstate.z && self.on_ground
        {
            transform.rotation.z -= 45.0*15.0*delta_time;
        }
        

        if moveleft && !(transform.position.x + transform.scale.x/2.0 < -1.8)
        {
            transform.position.x -= 1.0 * delta_time;
           
            
        }
        if moveright && !(transform.position.x - transform.scale.x/2.0 > 1.8)
        {
            transform.position.x += 1.0 * delta_time;
            
        }
        
        if jump
        {
            
            self.is_jumping = true;
            self.on_ground = false;

        }

        
    }


}