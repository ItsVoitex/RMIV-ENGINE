

use rmiv_engine::prelude::*;



pub struct Scene
{
    pub objects:Vec<Entity>,
    lines:Vec<String>,
    floor:f32,
    backrounds:(Entity,Entity,Entity)
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
    

    pub fn create(floor:f32,backrounds:(Entity,Entity,Entity)) -> Scene
    {
        Scene { 
            objects: Vec::new(),
            lines:Vec::new(),
            floor,
            backrounds        
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
        let temp = world.get_transfom(&self.backrounds.1) ;
        temp.position.x -= 0.4 **delta_time;
        if temp.position.x < -5.4
        {
            temp.position.x = 5.4
        }
        let temp = world.get_transfom(&self.backrounds.2) ;
        temp.position.x -= 0.4 **delta_time;
        if temp.position.x < -5.4
        {
            temp.position.x = 5.4
        }
    

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


    pub fn spawn_spikes(&mut self,spike_id:AssetHandle,world:&mut World,defaultmesh:&AssetHandle)
    {
        
        let mut offset = 2.2;
        for i in (0..self.objects.len()).rev()
        {
            if world.get_transfom(&self.objects[i]).position.x < -2.4
            {
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
                        offset +=0.27;
                    }
                    if self.lines[i].chars().nth(j) == Some('^')
                    {
                        offset += 0.27;
                        let object = world.spawn((Transform{
                            position:vec3(offset,self.floor-0.025,-0.6),
                            scale:vec3(0.27, 0.2, 0.2),
                            rotation: vec3(0.0,180.0,0.0), 
                        },
                        Render{
                            shader_id:None,
                            mesh_id:*defaultmesh,
                            texture_id:Some(spike_id),
                            renderlayers:(1,1),
                        },
                        Hitbox{
                            length:0.1, 
                            height:0.1,
                            width:0.1
                        }
                        ));
                        /*length:0.27, 
                            height:0.25,
                            width:0.25 */
                        
                        self.objects.push(object);
                    }
                    
                }
            }
        }
        
        
    }   
   
}
    
    


