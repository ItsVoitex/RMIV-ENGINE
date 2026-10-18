

use crate::{managers::window::WindowHandle, prelude::*};

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
}
pub struct Transform
{
    pub position:Vec3,
    pub scale:Vec3,
    pub rotation:f32,
}
pub struct Hitbox
{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}
pub struct Camera{
    pub position:Vec3,
    pub rotation:f32,
    pub renderlayers:(u32,u32),
    pub targetwindow:WindowHandle,
}
pub struct Render
{
    pub texture_id:Option<u32>,
    pub shader_id: u32,
    pub renderlayers:(u32,u32)
}

pub struct World
{
    next_id:u32,
    transforms : Vec<Option<Transform>>,
    hitboxes : Vec<Option<Hitbox>>,
    renders : Vec<Option<Render>>,
    cameras : Vec<Option<Camera>>
}
impl Component for Transform
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.transforms[*entity as usize] = Some(self);
    }
}
impl Component for Hitbox
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.hitboxes[*entity as usize] = Some(self);
    }
}
impl Component for Render
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.renders[*entity as usize] = Some(self);
    }
}
impl Component for Camera
{
    fn add_to(self, world: &mut World, entity: &Entity) {
        world.cameras[*entity as usize] = Some(self);
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

impl World
{
    pub fn new() -> World
    {
        World 
        { 
            next_id:0,
            transforms: Vec::new(), 
            hitboxes: Vec::new(), 
            renders: Vec::new(),
            cameras: Vec::new() 
        }
    }
    pub fn spawn<T:Component>(&mut self,components:T) -> Entity
    {
        let entity = self.next_id;
        self.transforms.push(None);
        self.hitboxes.push(None);
        self.renders.push(None);
        self.cameras.push(None);
        self.next_id += 1;
        components.add_to(self,&entity);
        entity
    }
    pub fn destroy(&mut self,entity:Entity)
    {
        self.transforms[entity as usize] = None;
        self.hitboxes[entity as usize] = None;
        self.renders[entity as usize] = None;
        self.cameras[entity as usize] = None;

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
            if let Some(render) = &self.renders[entities[i] as usize] && let Some(transform) = &self.transforms[entities[i] as usize]
            {
                vec.push((render,transform));
            }
            else {
                panic!("failed to get render components");
            }
        }
        return vec;
  
    }
    pub fn get_camera_components(&self,entities:Vec<Entity>) -> Vec<&Camera>
    {
        let mut vec:Vec<&Camera> = Vec::new();
        for i in 0..entities.len()
        {
            if let Some(camera) = &self.cameras[entities[i] as usize] 
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
        if let Some(component) = self.transforms[*entity as usize].as_mut()
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    pub fn get_hitbox(&mut self,entity:&Entity) -> &mut Hitbox
    {
        if let Some(component) = self.hitboxes[*entity as usize].as_mut()
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    pub fn get_render(&mut self,entity:&Entity) -> &mut Render
    {
        if let Some(component) = self.renders[*entity as usize].as_mut()
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
    pub fn get_camera(&mut self,entity:&Entity) -> &mut Camera
    {
        if let Some(component) = self.cameras[*entity as usize].as_mut()
        {
            return component;
        }
        else {
            panic!("failed to get component")
        }
    }
        

    pub fn query(&self,componenttype:ComponentType) -> Vec<u32>
    {
        match componenttype{
            ComponentType::Transform => {
                let mut val = Vec::new();
                for i in 0..self.transforms.len()
                {
                    if let Some(_) =  &self.transforms[i]
                    {
                        val.push(i as u32);
                    }
                    else {
                        continue;
                    }
                }
                return val;
            },         
            ComponentType::Hitbox=> {
                let mut val = Vec::new();
                for i in 0..self.hitboxes.len()
                {
                    if let Some(_) =  &self.hitboxes[i]
                    {
                        val.push(i as u32);
                    }
                    else {
                        continue;
                    }
                }
                return val;

            },
            ComponentType::Render => {
                let mut val = Vec::new();
                for i in 0..self.renders.len()
                {
                    if let Some(_) =  &self.renders[i]
                    {
                        val.push(i as u32);
                    }
                    else {
                        continue;
                    }
                }
                return val;

            },
            ComponentType::Camera =>{
                let mut val = Vec::new();
                for i in 0..self.cameras.len()
                {
                    if let Some(_) =  &self.cameras[i]
                    {
                        val.push(i as u32);
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
        if let Some(hitbox1) = &self.hitboxes[*entity as usize] && let Some(transform1) = &self.transforms[*entity as usize]
        {
            let tr = vec3( 0.5*hitbox1.x + transform1.position.x,   0.5*hitbox1.y + transform1.position.y,    0.5*hitbox1.z + transform1.position.z);
            let bl = vec3(-0.5*hitbox1.x + transform1.position.x,  -0.5*hitbox1.y + transform1.position.y,    0.5*hitbox1.z + transform1.position.z);
            for i in 0..entities.len(){
                if let Some(hitbox2) = &self.hitboxes[*entities[i] as usize] && let Some(transform2) = &self.transforms[*entities[i] as usize]
                {
                    let tr1 = vec3( 0.5*hitbox2.x + transform2.position.x,   0.5*hitbox2.y + transform2.position.y,    0.5*hitbox2.z + transform2.position.z);
                    let bl1 = vec3(-0.5*hitbox2.x + transform2.position.x,  -0.5*hitbox2.y + transform2.position.y,    0.5*hitbox2.z + transform2.position.z);
                    if (bl.x <= tr1.x && bl1.x <= tr.x) &&  (bl.y <= tr1.y && bl1.y <= tr.y) //&& (bl.z <= tr1.z && bl1.z <= tr.z)
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