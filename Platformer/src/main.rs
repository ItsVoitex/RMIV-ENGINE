
mod player;
mod scene;
use scene::Scene;
use player::Player;
use rmiv_engine::{backend::Mesh, managers::{asset::Texture}, prelude::*};
/*
max
-921.6
921.6

min
-504.9
504.9
*/
// screen split into 25
// 76.8 for gap

pub struct Game
{
    window:WindowHandle,
   
}
 pub fn gencube(width:f32,height:f32) -> Mesh
    {
         let vertices: Vec<Vec3>  = vec![ 
            vec3(width / 2.0,  -height / 2.0,   width/2.0),
            vec3(-width / 2.0,  -height / 2.0,  width/2.0),
            vec3(-width / 2.0,   height / 2.0,  width/2.0),
           vec3(width / 2.0,   height / 2.0,    width/2.0),

            vec3(width / 2.0,  -height / 2.0,    -width/2.0),
            vec3(-width / 2.0,  -height / 2.0,    -width/2.0),
            vec3(-width / 2.0,   height / 2.0,    -width/2.0),
           vec3(width / 2.0,   height / 2.0,    -width/2.0),
        ];
        let indices:Vec<u32>= vec!
        [
            0,  1,  2,
            0,  2,  3,

            4, 0, 3,
            4, 3, 7,

            5, 4, 7,
            5, 7, 6,

            1, 5, 6,
            1, 6, 2,

            4, 5, 1,
            4, 1, 0,

            3, 2, 6,
            3, 6, 7


        ];

        let texture_cords:Vec<Vec2> = vec![
            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,),

            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,)
        ];
        Mesh { vertices, indices, color: None, texture_cords:Some(texture_cords) }
    }
fn startup(assets:&mut AssetManager,world:&mut World,platform:&mut Platform) ->Game
{
    
   

    let window = platform.create_window(Window{
        title:"zombiedash".to_string(),
        dimensions:(1920, 1080),
        targetfps:60,
        is_primary:false,
        windowref:None
    });
   

  
    



  
    
    
    let speed = 800.0;

      Game { 
        window, 
    }
        
}



fn main()
{
    App::new()
    .add_startup_system(startup)
    .add_update_system(update1)
    .run();

    
}
 
fn update1(_assets:&mut AssetManager,world:&mut World,platform:&mut Platform,game:&mut Game)
{
    
}
