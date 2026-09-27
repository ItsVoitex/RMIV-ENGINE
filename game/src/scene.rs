use std::ffi::CString;

use glfw::PWindow;
use glfw::WindowEvent::Iconify;
use rmiv_engine::Object;
use rmiv_engine::ObjectData;



pub struct Scene
{
    pub x:f32,
    pub y:f32,
    z:f32,
    pub objects:Vec<Object>,
    lines:Vec<String>,
    pub line_count:usize,
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
            x: 0.0, 
            y: 0.0, 
            z: 0.0, 
            objects: object,
            lines:Vec::new(),
            line_count:0,
        }
    }
    pub fn move_all(&mut self,delta_time:&f32)
    {
        for i in 0..self.objects.len()
        {
            if self.objects[i].object_data.object_type != "backround"
            {
                self.objects[i].x -= 800.0 * *delta_time 
            }
            
        }
    }
    pub fn draw(&mut self)
    {
        for i in 0..self.objects.len()
            {
                self.objects[i].draw();
            }
    }

    pub fn spawn_spikes(&mut self)
    {
    
        let obdat = ObjectData::create(76.8,70.0,-0.5,&["Assets/textures/spike.png"],String::from("spike"));

        for i in 0..20
        {
            self.objects.push(Object::multi_create(&obdat,String::from("spike")));
            self.objects[i].x = 960.0;
            self.objects[i].y = 520.0;
        }
        self.lines = read_lines("Assets/level1/level.dat");
    }
    pub fn move_spikes(&mut self)
    {
        let mut counter = 0;
        for i in 0..5
        {
            if self.objects[i].x < -960.0
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
                            self.objects[k].x = -960.0 + (76.8 * j as f32);
                        }
                        
                    }
                }
            }
            
        }
            
        
    
    }   
   
}
    
    


