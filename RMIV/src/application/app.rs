






use crate::{backend::*};


pub struct App<T>
{
    pub assets:AssetManager,
    pub world:World,
    pub renderer: Renderer,
    pub platform:Platform,
    startup_system:Option<fn(&mut AssetManager,&mut World,&mut Platform) -> T>,
    update_systems:Vec<fn(&mut AssetManager,&mut World,&mut Platform,&mut T)>,
    maxtickrate:i32,
    
}
impl <T>App<T>
{
    pub fn new() -> App<T>
    {
        
        let platform = Platform::new();
        let assets = AssetManager::new();
        App {
            renderer:Renderer::create(),
            world:World::new(),
            platform,
            assets:assets, 
            update_systems:Vec::new(),
            startup_system:None,
            maxtickrate:240,

            
        }
        
    }

    pub fn add_startup_system(&mut self,func:fn(&mut AssetManager,&mut World,&mut Platform) -> T) -> &mut App<T>
    {
        self.startup_system = Some(func);
        self
    }
    pub fn add_update_system(&mut self,func:fn(&mut AssetManager,&mut World,&mut Platform,&mut T)) -> &mut App<T>
    {
        self.update_systems.push(func);
        self
    }
    pub fn set_tickrate(&mut self,tickrate:i32)-> &mut App<T>
    {
        self.maxtickrate = tickrate;
        self
    }
    
    pub fn run(&mut self) -> &mut App<T>
    {    
        
        
        
        let gametime = 1.0/self.maxtickrate as f64;
    
        
        let mut tickrate = 0;

        let ussystems = std::mem::take(&mut self.update_systems);
        let assets = &mut self.assets;
        let renderer = &mut self.renderer;
        let platform = &mut self.platform;
        let world = &mut self.world;
        let mut state:Option<T> = None;
        if let Some(sssystems) = std::mem::take(&mut self.startup_system){
            state = Some(sssystems(assets,world,platform));
        }
        
        let mut lasttime2 = platform.runtime.get_time();
        let mut lasttime3 = platform.runtime.get_time();
      
        
        
        loop
        {
             
            let currenttime = platform.runtime.get_time();
            
            
            if platform.windowmanager.storage.should_close()
            {
                break;
            }
            
            if currenttime - lasttime3 >= 1.0
            {
                println!("{} tps",tickrate);
                println!("");
                tickrate = 0;
                lasttime3 = currenttime;
                
            }
            
            
            platform.update_events();
            if currenttime - lasttime2 >= gametime
            {
                
                world.update_cameras(platform);
                match &mut state {
                    Some(state) => {
                        for i in ussystems.iter()
                        {
                            i(assets,world,platform,state);
                        }
                    },
                    None => {},    
                
                };
                lasttime2 = currenttime;
                tickrate +=1;
            }
            
            Renderer::queue_draw(renderer,world,platform,assets);
            
        
            if platform.windowmanager.storage.should_close()
            {
                break;
            }
           
        
            
          
            
            
           
           
        }platform.windowmanager.storage.should_close();

        self
        
    }    
    

   
    
}
    
