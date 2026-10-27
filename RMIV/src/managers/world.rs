


use glam::Mat4;

use crate::{managers::window::{WindowHandle, WindowInstance}, prelude::*};

//let the user enter theeir own strcut values when doing add

pub trait Component {
    fn add_to(self, world: &mut World, entity: &Entity);
}
#[derive(PartialEq)]
pub enum ComponentType {
    Transform = 0,
    Hitbox = 1,
    Render = 2,
    Camera = 3,
    //Window = 4
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,)]
pub struct Entity{
    id:u32,
    generation:u32,
}
impl Entity{
    pub fn empty() ->Entity
    {
        Entity { id: 0, generation: 0 }
    }
    pub fn printdata(&self)
    {
        println!("id:{} generation:{}",self.id,self.generation);
    }
}
#[derive(Copy, Clone,PartialEq,Debug,)]
pub struct Transform
{
    pub position:Vec3,
    pub scale:Vec3,
    pub rotation:Vec3,
}
#[derive(Copy, Clone,PartialEq,Debug,)]
pub struct Hitbox
{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}
#[derive(Copy, Clone)]
pub struct Camera{
    pub position:Vec3,
    pub rotation:Vec3,
    pub fov:f32,
    pub renderlayers:(u32,u32),
    pub targetwindow:WindowHandle,
    pub view:Mat4,
    pub model:Mat4,
    pub proj:Mat4,
    pub is_2d:bool,    
}
impl Camera
{
    pub fn camera2d() ->Camera
    {
        let model = glam::Mat4::IDENTITY;
        let view = glam::Mat4::IDENTITY;
        let proj = glam::camera::rh::proj::opengl::orthographic(
        -960.0, 960.0,   
        -540.0, 540.0,   
        0.1, 100.0,      
        );
      

        Camera { 
            position: vec3(0.0, 0.0, 0.0), 
            rotation: vec3(0.0,0.0,0.0),
            fov:0.0,
            renderlayers: (0,0), 
            targetwindow: WindowHandle { id: 0, generation: 0, is_primary: false }, 
            view, 
            model, 
            proj,
            is_2d:true
        }
    }
    pub fn camera3d() -> Camera
    {
        let model = glam::Mat4::IDENTITY;
        let view: Mat4 =  glam::Mat4::IDENTITY;
        let proj: Mat4 =  glam::camera::rh::proj::opengl::perspective(60.0_f32.to_radians(), 1920.0/1080.0, 1.0, 100.0);
    
        Camera { 
            position: vec3(0.0, 0.0, 0.0), 
            rotation: vec3(0.0,0.0,0.0), 
            renderlayers: (0,0), 
            fov:60.0,
            targetwindow: WindowHandle { id: 0, generation: 0, is_primary: false }, 
            view, 
            model, 
            proj,
            is_2d:false
        }
    }
    pub fn updatecamera(&mut self,window:&WindowInstance)
    {
        if !self.is_2d
        {   
            self.proj = glam::camera::rh::proj::opengl::perspective(self.fov.to_radians(), window.frame_buffer.0 as f32/window.frame_buffer.1 as f32, 1.0, 100.0);
        }
        self.view = glam::Mat4::from_translation(self.position);
    }
    pub fn make3d(&mut self,fov:f32)
    {
        self.fov = fov;
        self.is_2d = false;
        self.proj = glam::camera::rh::proj::opengl::perspective(self.fov.to_radians(), 1920.0/1080 as f32, 1.0, 100.0);
    }
    pub fn make2d(&mut self)
    {
        self.is_2d = true;
        self.proj  = glam::camera::rh::proj::opengl::orthographic(
        -960.0, 960.0,   
        -540.0, 540.0,   
        0.1, 100.0,      
        ); 
    }
}
#[derive(Copy, Clone,PartialEq,Debug,)]
pub struct Render
{
    pub texture_id:Option<AssetHandle>,
    pub mesh_id:AssetHandle,
    pub shader_id: Option<AssetHandle>,
    pub renderlayers:(u32,u32)
}



struct Storage<T> {
    entities: Vec<Entity>,
    pub components: Vec<T>,
    entity_to_index: Vec<Option<usize>>,
}
pub struct World
{
    next_id:u32,
    freeentites:Vec<Entity>,
    transforms : Storage<Transform>,
    hitboxes : Storage<Hitbox>,
    renders : Storage<Render>,
    cameras : Storage<Camera>,
}
impl Component for Transform
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.transforms.entities.push(entity.clone());
        world.transforms.components.push(self);
        world.transforms.entity_to_index[entity.id as usize] = Some(world.transforms.components.len() -1);
    }
}

impl Component for Hitbox
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.hitboxes.entities.push(entity.clone());
        world.hitboxes.components.push(self);
        world.hitboxes.entity_to_index[entity.id as usize] = Some(world.hitboxes.components.len() -1);
    }
}
impl Component for Render
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.renders.entities.push(entity.clone());
        world.renders.components.push(self);
        world.renders.entity_to_index[entity.id as usize] = Some(world.renders.components.len() -1);
    }
}
impl Component for Camera
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.cameras.entities.push(entity.clone());
        world.cameras.components.push(self);
        world.cameras.entity_to_index[entity.id as usize] = Some(world.cameras.components.len() -1);
    }
}
impl<A:Component ,B:Component> Component for (A,B)
{
    fn add_to(self, world: &mut World, entity: &Entity)
    {
        self.0.add_to(world, entity);
        self.1.add_to(world, entity);
    }
}
impl<A:Component ,B:Component,C:Component> Component for (A,B,C)
{
    fn add_to(self, world: &mut World, entity: &Entity)
    {
        self.0.add_to(world, entity);
        self.1.add_to(world, entity);
        self.2.add_to(world, entity);
    }
}
impl<A:Component ,B:Component,C:Component,D:Component> Component for (A,B,C,D)
{
    fn add_to(self, world: &mut World, entity: &Entity)
    {
        self.0.add_to(world, entity);
        self.1.add_to(world, entity);
        self.2.add_to(world, entity);
        self.3.add_to(world, entity);
    }
}

impl <T>Storage<T>
{
    pub fn new() -> Storage<T>
    {
        Storage { entities: Vec::new(), components: Vec::new(), entity_to_index: Vec::new() }
    }
    pub fn get_component(&self,entity:&Entity) -> Option<&T>
    {
        if let Some(index) = self.entity_to_index[entity.id as usize]
        {
            if self.entities[index] == *entity
            {
                return Some(&self.components[index]);
            }
            return None;
        }
        return None;
        
    }
    pub fn get_mut_component(&mut self,entity:&Entity) -> Option<&mut T>
    {
        if let Some(index) = self.entity_to_index[entity.id as usize]
        {
            if self.entities[index] == *entity
            {
                return Some(&mut self.components[index]);
            }
            return None;
        }
        return None;     
    }
    pub fn destroy(&mut self,entity:&Entity)
    {
        let index = self.entity_to_index[entity.id as usize];
        match index {
            Some(pos) => {
                self.entity_to_index[self.entities[self.entities.len() -1].id as usize] = Some(pos);
                self.entities.swap_remove(pos);
                self.components.swap_remove(pos);
                self.entity_to_index[entity.id as usize] = None;
                
            },
            None =>{},
        }
    }
    
}

impl World
{
    pub fn new() -> World
    {
        World 
        { 
            next_id:0,
            freeentites:Vec::new(),
            transforms: Storage::new(), 
            hitboxes: Storage::new(), 
            renders: Storage::new(),
            cameras: Storage::new(),
        }
    }
    pub fn update_cameras(&mut self,platform:&mut Platform)
    {
       for i in self.cameras.components.iter_mut() 
       {
            let window = platform.windowmanager.get(&i.targetwindow, false);
            if window.is_ok()
            {
                i.updatecamera(window.unwrap())
            }  
       }
        
    }
    pub fn spawn<T:Component>(&mut self,components:T) -> Entity
    {
        if self.freeentites.len() > 0
        {
            let mut entity = self.freeentites.remove(0);
            entity.generation += 1;
            components.add_to(self,&entity);
            entity
        }
        else 
        {
            let entity = Entity { id: self.next_id, generation: 0 };
            self.transforms.entity_to_index.push(None);
            self.hitboxes.entity_to_index.push(None);
            self.renders.entity_to_index.push(None);
            self.cameras.entity_to_index.push(None);
            self.next_id += 1;
            components.add_to(self,&entity);
            entity
        }
        
    }
    pub fn spawn_empty(&mut self) -> Entity
    {
        if self.freeentites.len() > 0
        {
            let mut entity = self.freeentites.remove(0);
            entity.generation += 1;
            entity
        }
        else 
        {
            let entity = Entity { id: self.next_id, generation: 0 };
            self.transforms.entity_to_index.push(None);
            self.hitboxes.entity_to_index.push(None);
            self.renders.entity_to_index.push(None);
            self.cameras.entity_to_index.push(None);
            self.next_id += 1;
            entity
        }
        
    }
    pub fn destroy(&mut self,entity:Entity)
    {
        self.transforms.destroy(&entity);
        self.cameras.destroy(&entity);
        self.hitboxes.destroy(&entity);
        self.renders.destroy(&entity);
        self.freeentites.push(entity);
    }
    pub fn clone(&mut self,entity:&Entity) -> Entity
    {
        let new = self.clone_all_components(entity);
        let entity = self.spawn_empty();
        if new.0.is_some()
        {
            self.add_component(&entity, new.0.unwrap());
        }
        if new.1.is_some()
        {
            self.add_component(&entity, new.1.unwrap());
        }
        if new.2.is_some()
        {
            self.add_component(&entity, new.2.unwrap());
        }
        if new.3.is_some()
        {
            self.add_component(&entity, new.3.unwrap());
        }
        entity

    }
    pub fn add_component<T:Component>(&mut self,entity:&Entity,component:T)
    {
       component.add_to(self,entity)
    }
    pub fn get_render_components(&self,entities:Vec<Entity>) -> Vec<(&Render,&Transform)>
    {
        let mut vec:Vec<(&Render,&Transform)> = Vec::new();
        for i in 0..entities.len()
        {
            if let Some(render) = self.renders.get_component(&entities[i]) && let Some(transform) = self.transforms.get_component(&entities[i])
            {
                vec.push((render,transform));
            }
            else {
                panic!("failed to get render components");
            }
        }
        return vec;
  
    }
    fn clone_all_components(&mut self,entity:&Entity) -> (Option<Render>,Option<Transform>,Option<Camera>,Option<Hitbox>)
    {
        let mut temp:(Option<Render>,Option<Transform>,Option<Camera>,Option<Hitbox>) = (None,None,None,None);
        if let Some(render) = self.renders.get_component(entity)
        {
            temp.0 = Some(render.clone());
        }
        if let Some(transform) = self.transforms.get_component(entity)
        {
            temp.1 = Some(transform.clone());
        }
        if let Some(camera) = self.cameras.get_component(entity)
        {
            temp.2 = Some(camera.clone());
        }
        if let Some(hitbox) = self.hitboxes.get_component(entity)
        {
            temp.3 = Some(hitbox.clone());
        }
        temp
    }
    pub fn get_all_components(&mut self,entity:&Entity) -> (Option<&Render>,Option<&Transform>,Option<&Camera>,Option<&Hitbox>)
    {
        let mut temp:(Option<&Render>,Option<&Transform>,Option<&Camera>,Option<&Hitbox>) = (None,None,None,None);
        if let Some(render) = self.renders.get_component(entity)
        {
            temp.0 = Some(render);
        }
        if let Some(transform) = self.transforms.get_component(entity)
        {
            temp.1 = Some(transform);
        }
        if let Some(camera) = self.cameras.get_component(entity)
        {
            temp.2 = Some(camera);
        }
        if let Some(hitbox) = self.hitboxes.get_component(entity)
        {
            temp.3 = Some(hitbox);
        }
        temp

    }
    pub fn get_camera_components(&self,entities:Vec<Entity>) -> Vec<&Camera>
    {
        let mut vec:Vec<&Camera> = Vec::new();
        for i in 0..entities.len()
        {
            if let Some(camera) = self.cameras.get_component(&entities[i]) 
            {
                vec.push(camera);
            }
            else {
                panic!("failed to get render components");
            }
        }
        return vec;
  
    }
    

    pub fn get_transfom(&mut self,entity:&Entity) -> &mut Transform
    {
        if let Some(component) = self.transforms.get_mut_component(entity)
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    pub fn get_hitbox(&mut self,entity:&Entity) -> &mut Hitbox
    {
        if let Some(component) = self.hitboxes.get_mut_component(&entity)
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    pub fn get_render(&mut self,entity:&Entity) -> &mut Render
    {
        if let Some(component) = self.renders.get_mut_component(&entity)
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    pub fn get_camera(&mut self,entity:&Entity) -> &mut Camera
    {
        if let Some(component) = self.cameras.get_mut_component(&entity)
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    /*pub fn get_window(&mut self,entity:&Entity) -> &Window
    {
        if let Some(component) = self.windows.get_mut_component(&entity)
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    */
        

    pub fn query(&self,componenttype:ComponentType) -> Vec<Entity>
    {
        match componenttype{
            ComponentType::Transform => {
                let mut val = Vec::new();
                for i in 0..self.transforms.entity_to_index.len()
                {
                    if let Some(index) =  &self.transforms.entity_to_index[i]
                    {
                        val.push(self.transforms.entities[*index].clone());
                    }
                    else {
                        continue;
                    }
                }
                return val;
            },         
            ComponentType::Hitbox=> {
                let mut val = Vec::new();
                for i in 0..self.hitboxes.entity_to_index.len()
                {
                    if let Some(index) =  &self.hitboxes.entity_to_index[i]
                    {
                        val.push(self.hitboxes.entities[*index].clone());
                    }
                    else {
                        continue;
                    }
                }
                return val;

            },
            ComponentType::Render => {
                let mut val = Vec::new();
                for i in 0..self.renders.entity_to_index.len()
                {
                    if let Some(index) =  &self.renders.entity_to_index[i]
                    {
                        val.push(self.renders.entities[*index].clone());
                    }
                    else {
                        continue;
                    }
                }
                return val;

            },
            ComponentType::Camera =>{
                let mut val = Vec::new();
                for i in 0..self.cameras.entity_to_index.len()
                {
                    if let Some(index) =  &self.cameras.entity_to_index[i]
                    {
                        val.push(self.cameras.entities[*index].clone());
                    }
                    else {
                        continue;
                    }
                }
                return val;
            }
           

        }
    }
    pub fn check_collision(&mut self,entity:&Entity,entities:&Vec<&Entity>,skip_entities_with_no_hitbox:bool) ->bool
    {
        if let Some(hitbox1) = self.hitboxes.get_component(entity) && let Some(transform1) = self.transforms.get_mut_component(entity)
        {
            let meshpos = applyrotation(vec2(0.5, -0.5), transform1.rotation.x);
            let tr = vec3( meshpos.x*hitbox1.x + transform1.position.x,   meshpos.x*hitbox1.y + transform1.position.y,    meshpos.x*hitbox1.z + transform1.position.z);
            let bl = vec3(meshpos.y*hitbox1.x + transform1.position.x,  meshpos.y*hitbox1.y + transform1.position.y,    meshpos.y*hitbox1.z + transform1.position.z);
            for i in 0..entities.len(){
                if let Some(hitbox2) = self.hitboxes.get_mut_component(&entities[i]) && let Some(transform2) = self.transforms.get_mut_component(&entities[i])
                {
                    let meshpos2 = applyrotation(vec2(0.5, -0.5), transform2.rotation.x);
                    let tr1 = vec3( meshpos2.x*hitbox2.x + transform2.position.x,   meshpos2.x*hitbox2.y + transform2.position.y,    meshpos2.x*hitbox2.z + transform2.position.z);
                    let bl1 = vec3(meshpos2.y*hitbox2.x + transform2.position.x,  meshpos2.y*hitbox2.y + transform2.position.y,    meshpos2.y*hitbox2.z + transform2.position.z);
                    if  (bl.x <= tr1.x && bl1.x <= tr.x) &&  (bl.y <= tr1.y && bl1.y <= tr.y) && (bl.z <= tr1.z && bl1.z <= tr.z)
                    {
                        return true;
                    }
                    continue;
                }
                if skip_entities_with_no_hitbox{
                    continue;
                }
                else {
                    panic!("comparison object to check collisions has no hitbox compononent and skipping objects with no hitbox is not enabled")
                } 
            }
            return false;
        }
        else {
            panic!("main object has no hitbox component failure to check collision");
        }

    }
    
}

pub fn applyrotation(pos:Vec2,angle:f32) ->Vec2
{
    let angle = f32::to_radians(angle);
    let s = f32::sin(angle);
    let c = f32::cos(angle);
    let x = glam::Mat2::from_cols(vec2(c, -s),vec2(s ,c)) *pos;
    return x;
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