






use crate::{backend::*};
use std::time::{Duration, Instant};

pub struct App<T>
{
    startup_system:Option<fn(&mut AssetManager,&mut World,&mut Platform) -> T>,
    update_systems:Vec<fn(&mut AssetManager,&mut World,&mut Platform,&mut T)>,
    maxtickrate:i32,
    enabledebug:bool
    
}
impl <T>App<T>
{
    pub fn new() -> App<T>
    {
        
        
        App {
            update_systems:Vec::new(),
            startup_system:None,
            maxtickrate:200,
            enabledebug:false

            
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
    pub fn toggle_debug(&mut self) -> &mut App<T>
    {
        if self.enabledebug
        {
            self.enabledebug = false
        }
        else {
            self.enabledebug = true;
        }
        self
        
    }
    pub fn run(&mut self) -> &mut App<T>
    {    
        
       
        
        let gametime = Duration::from_secs(1)/self.maxtickrate as u32;
    
        
        let mut tickrate = 0;

        let ussystems = std::mem::take(&mut self.update_systems);

        let mut platform = Platform::new();
        let mut assets = AssetManager::new();
        let mut world = World::new();
        


        let mut state:Option<T> = None;
        if let Some(sssystems) = std::mem::take(&mut self.startup_system){
            state = Some(sssystems(&mut assets,&mut world,&mut platform));
        }
        
        let mut renderer = Renderer::create(self.enabledebug);
        let mut last_time1 = Instant::now();
        let mut last_time2 = Instant::now();
      
       
        
        loop
        {
             
            let currenttime = Instant::now();
            
            
            if platform.windowmanager.storage.should_close()
            {
                break;
            }
            
            if currenttime.duration_since(last_time1) >= Duration::from_secs(1)
            {
                println!("{} tps",tickrate);
                println!("");
                tickrate = 0;
                last_time1 = currenttime;
                
            }
            
            
            platform.update_events();
            
            if currenttime.duration_since(last_time2) >= gametime
            {
                
                world.update_cameras(&mut platform);
                match &mut state {
                    Some(state) => {
                        for i in ussystems.iter()
                        {
                            i(&mut assets,&mut world,&mut platform,state);
                        }
                    },
                    None => {},    
                
                };
                last_time2 = currenttime;
                tickrate +=1;
                world.update_snapshot(&mut renderer.rendersnapshot);
                world.update_camera_snapshot(&mut renderer.cameras);
                renderer.queue_draw(&mut platform.windowmanager,&mut assets,&mut world);
               
            }
            else {
                platform.runtime.wait_events_timeout((gametime - (currenttime - last_time2)).as_secs_f64() );
            }
            
            
           
            
        
            if platform.windowmanager.storage.should_close()
            {
                break;
            }
           
 
           
        }platform.windowmanager.storage.should_close();
        
        self
        
    }    
    

   
    
}
    
