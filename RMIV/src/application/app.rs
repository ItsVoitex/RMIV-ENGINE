



use crate::backend::*;


pub struct App<T>
{
    pub assets:AssetManager,
    pub world:World,
    pub renderer: Renderer,
    pub platform:Platform,
    startup_system:Option<fn(&mut AssetManager,&mut World,&mut Platform) -> T>,
    update_systems:Vec<fn(&mut AssetManager,&mut World,&mut Platform,&mut T)>,
    
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
    
    pub fn run(&mut self) -> &mut App<T>
    {    
        
        let ussystems = std::mem::take(&mut self.update_systems);
        let assets = &mut self.assets;
        let renderer = &mut self.renderer;
        let platform = &mut self.platform;
        let world = &mut self.world;
        let mut state:Option<T> = None;
        if let Some(sssystems) = std::mem::take(&mut self.startup_system){
            state = Some(sssystems(assets,world,platform));
        } 
    
        loop
        {
            platform.update_events();
            if !platform.windowmanager.storage.should_close()
            {
                break;
            }
            world.update_cameras(platform);
            match &mut state {
                Some(state) => {
                    for i in ussystems.iter()
                    {
                        i(assets,world,platform,state);
                    }
                },
                None => {},      
            }
            if !platform.windowmanager.storage.should_close()
            {
                break;
            }
            Renderer::queue_draw(renderer,world,platform,assets)
            
            
        }platform.windowmanager.storage.should_close();

        self
        
    }    
    

   
    
}
    
