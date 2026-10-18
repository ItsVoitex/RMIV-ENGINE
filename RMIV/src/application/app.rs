
use glam::u32;


use crate::prelude::*;


use crate::application::app::SystemType::{Startup, Update};
use crate::managers::platform::Platform;
use crate::managers::window::{WindowHandle};
use crate::renderer::{Renderer};





#[derive(PartialEq)]
pub enum SystemType {
    Update = 0,
    Startup = 1,
    
}

type SystemFn = fn(&mut AssetManager,&mut World,&mut Platform);
pub struct App<T>
{
    pub assets:AssetManager,
    pub world:World,
    pub renderer: Renderer,
    pub platform:Platform,
    startup_systems:Vec<SystemFn>,
    update_systems:Vec<SystemFn>,
    startup_state_system:Option<fn(&mut AssetManager,&mut World,&mut Platform) -> T>,
    update_state_systems:Vec<fn(&mut AssetManager,&mut World,&mut Platform,&mut T)>,
    
}
impl <T>App<T>
{
    pub fn new() -> App<T>
    {
        
        let platform = Platform::new();
        let mut assets = AssetManager::new();
        assets.load_default_shader();
        App {
            renderer:Renderer::create(),
            world:World::new(),
            platform,
            assets:assets, 
            update_systems:Vec::new(),
            startup_systems:Vec::new(),
            startup_state_system:None,
            update_state_systems:Vec::new()
            
        }
        
    }
    
    
    pub fn add_system(&mut self,systemtype:SystemType,func:SystemFn) -> &mut App<T>
    {
        
        if systemtype == Startup
        {
            self.startup_systems.push(func);
        }
        if systemtype == Update
        {
            self.update_systems.push(func);
        }
        
        self
    }
    pub fn add_startupstate_system(&mut self,func:fn(&mut AssetManager,&mut World,&mut Platform) -> T) -> &mut App<T>
    {
        self.startup_state_system = Some(func);
        self
    }
    pub fn add_updatestate_system(&mut self,func:fn(&mut AssetManager,&mut World,&mut Platform,&mut T)) -> &mut App<T>
    {
        self.update_state_systems.push(func);
        self
    }
    
    
    pub fn run(&mut self) -> &mut App<T>
    {
        let ssystems = std::mem::take(&mut self.startup_systems);
        let usystems = std::mem::take(&mut self.update_systems);      
        
        let ussystems = std::mem::take(&mut self.update_state_systems);
        let assets = &mut self.assets;
        let renderer = &mut self.renderer;
        let platform = &mut self.platform;
        let world = &mut self.world;
        let mut state:Option<T> = None;
        if let Some(sssystems) = std::mem::take(&mut self.startup_state_system){
            state = Some(sssystems(assets,world,platform));
        } 
        for i in ssystems.iter()
        {
            i(assets,world,platform);
        }
        
    
        while platform.windows.sharedwindows.iter().any(|wind| wind.is_some()) || platform.windows.primarywindows.iter().any(|wind| wind.is_some())
        {
            
            platform.update_events();
            for i in usystems.iter()
            {
                i(assets,world,platform)
            }
            match &mut state {
                Some(state) => {
                    for i in ussystems.iter()
                    {
                        i(assets,world,platform,state);
                    }
                },
                None => {},      
            }
            Renderer::queue_draw(renderer,world,platform,assets);
            
            
        }

        self
        
    }


    pub fn should_close(&mut self,windowhandle:&WindowHandle) ->bool
    {
        let window = 
        match self.platform.windows.get_mut(&windowhandle){
            Some(wind) => wind,
            None => return true,
        };
        return window.should_close();
    }
    
     
    pub fn load_textured_mesh(&mut self,texture_path:&str,width:f32,height:f32,depth:f32,window_handle:WindowHandle) -> Option<(u32,u32)>
    {
       let window = match self.platform.windows.get_mut(&window_handle){
            Some(wind) => wind,
            None => return None,
        };
        window.make_current();
        Some(self.assets.load_textured_mesh(texture_path, width, height, depth))
        
    }
    pub fn load_texture_from_file(&mut self,texture_path:&str) -> u32
    {
        self.assets.load_texture_from_file(texture_path, )
    }

    pub fn load_shader_from_file(&mut self,shader_path:&[&str;2]) -> u32
    {
        self.assets.load_shader_from_file(shader_path)
    }


    pub fn load_default_shader(&mut self) -> u32
    {
        self.assets.load_default_shader()
    }

    

   
    
}
    
