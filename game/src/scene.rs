use rmiv_engine::prelude::*;

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
        Scene { 
            objects: Vec::new(),
            lines:Vec::new(),
        }
    }
    pub fn move_all(&mut self,delta_time:&f32)
    {
        for j in 0..self.objects.len()
        {
            self.objects[j].position.x -= 800.0 * *delta_time;
            
        }
    }
    pub fn draw<T>(&mut self,app:&mut App<T>,windowid:u32)
    {
        for i in 0..self.objects.len()
            {
                app.draw(&mut self.objects[i], windowid);
            }
    }
    pub fn checkleveldat(&mut self)
    {
        self.lines = read_lines("Assets/level1/level.dat");
    }




    pub fn spawn_spikes(&mut self,spike_id:u32)
    {
        
        let mut offset = 960.0;
        for i in (0..self.objects.len()).rev()
        {
            if self.objects[i].position.x < -960.0
            {
                self.objects.swap_remove(i);
            }
            
        }
        if self.objects.len() <= 3
        {
            for i in 0..self.lines.len()
            {
                for j in 0..25
                {
                    if self.lines[i].chars().nth(j) == Some('*')
                    {
                        offset +=115.2;
                    }
                    if self.lines[i].chars().nth(j) == Some('^')
                    {
                        offset += 115.2;
                        let mut object = Object::create(spike_id,76.8*1.5, 70.0*1.3, -0.5, 0);
                        object.position.x += offset;
                        object.position.y = 520.0;
                        self.objects.push(object);
                    }
                    
                }
            }
        }
        
        
    }   
   
}
    
    


