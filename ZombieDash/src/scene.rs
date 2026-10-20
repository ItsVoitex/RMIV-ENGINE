

use rmiv_engine::prelude::*;



pub struct Scene
{
    pub objects:Vec<Entity>,
    lines:Vec<String>,
}

fn read_lines(file_path:&str) -> Vec<String>
    {
        let mut result = Vec::new();
        let file = std::fs::read_to_string(file_path);
        let file = match file{
            Ok(file) =>file,
            _ =>{
                let _ = std::fs::write("Assets/level1/level.dat", "^*****^*****^***^**^^***^;\n*****^***^*******^^****^*;\n*****^^*****^*****^****^*;\n*****^^****^****^*******^;\n******^*****^**^*******^^;\n******^****^**^***^^***^*;\n****^^******^****^***^***;\n*^***^****^***^^****^****;");
                let file = std::fs::read_to_string("Assets/level1/level.dat").expect("failed to create or read file path:Assets/level1/level.dat");
                file
            }
        };
        for line in file.lines() 
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
    pub fn getobjects(&mut self) -> Vec<&Entity>
    {
        let mut vec = Vec::new();
        for i in 0..self.objects.len()
        {   
            vec.push(&self.objects[i]);
        }
        return vec;
    }
    pub fn move_all(&mut self,delta_time:&f32,speed:f32,world:&mut World)
    {
        for j in 0..self.objects.len()
        {
            world.get_transfom(&self.objects[j]).position.x -= speed * *delta_time;
            
        }
    }
    pub fn checkleveldat(&mut self)
    {
        self.lines = read_lines("Assets/level1/level.dat");
    }


    pub fn reset(&mut self,world:&mut World)
    {
        for i in (0..self.objects.len()).rev()
        {
            world.destroy(self.objects[i]);
            self.objects.swap_remove(i);
        }
    }


    pub fn spawn_spikes(&mut self,spike_id:u32,world:&mut World)
    {
        
        let mut offset = 960.0;
        for i in (0..self.objects.len()).rev()
        {
            if world.get_transfom(&self.objects[i]).position.x < -1000.0
            {
                self.objects[i].printdata();
                world.destroy(self.objects[i]);
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
                        let object = world.spawn((Transform{
                            position:vec3(0.0,-520.0,0.0),
                            scale:vec3(76.8*1.5, 70.0*1.3, -0.5),
                            rotation:0.0
                        },
                        Render{
                            shader_id:0,
                            texture_id:Some(spike_id),
                            renderlayers:(1,1),
                        },
                        Hitbox{
                            x:76.8, 
                            y:70.09,
                            z:-0.5
                        }
                        ));
                        
                        
                        let transform = world.get_transfom(&object);
                        transform.position.x += offset;
                        self.objects.push(object);
                    }
                    
                }
            }
        }
        
        
    }   
   
}
    
    


