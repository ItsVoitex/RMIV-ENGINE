
use rmiv_engine::AssetManager;
use rmiv_engine::Object;

use rmiv_engine::App;


pub struct Scene
{
    pub objects:Vec<Object>,
    lines:Vec<String>,
}

fn read_lines(file_path:&str) -> Vec<String>
    {
        let mut result = Vec::new();
        for line in std::fs::read_to_string(file_path).unwrap().lines() 
        {
            result.push(line.to_string())
        }
        result
    
    }
impl Scene
{
    

    pub fn create() -> Scene
    {
        let object :Vec<Object> = Vec::new();
        Scene { 
            objects: object,
            lines:Vec::new(),
        }
    }
    pub fn move_all(&mut self,delta_time:&f32)
    {
        for i in 0..self.objects.len()
        {
            self.objects[i].position.x -= 800.0 * *delta_time;
            
        }
    }
    pub fn draw(&mut self,app:&mut App)
    {
        for i in 0..self.objects.len()
            {
                self.objects[i].draw(app);
            }
    }

    pub fn spawn_spikes(&mut self,assetmanager:&mut AssetManager)
    {
        assetmanager.load_textured_mesh("Assets/textures/spike.png", 76.8, 70.0, -0.5, "spike");

        for i in 0..20
        {
            self.objects.push(Object::create("spike","spike","default"));
            self.objects[i].position.x = 960.0;
            self.objects[i].position.y = 520.0;
        }
        self.lines = read_lines("Assets/level1/level.dat");
    }


    pub fn move_spikes(&mut self)
    {
        let mut counter = 0;
        for i in 0..5
        {
            if self.objects[i].position.x < -960.0
            {
                counter +=1;
            }
            
        }
        if counter >= 5
        {
            for i in 0..5
            {
                for  j in 0..25 
                {
                    for k in 0..5
                    {
                        if self.lines[i].chars().nth(j) == Some('^')
                        {
                            self.objects[k].position.x = -960.0 + (76.8 * j as f32);
                        }
                        
                    }
                }
            }
            
        }
            
        
    
    }   
   
}
    
    


