mod player;
use std::ffi::{CString, c_void};
use glfw::{Context, Key::Space, fail_on_errors};
use gl;
use player::Player;
mod scene;
use rmiv_engine;
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



fn main()
{

    
    let width = 2560;
    let height = 1440;

    let mut App = rmiv_engine::App::new();
    App.create_window(width, height, "Zombie Dash","hello");
  
    


    
        
    let mut backround = rmiv_engine::Object::create(1920.0, 1080.0, 0.0, &["Assets/textures/backround.png"]);
    
    
    let mut Scene1 = scene::Scene::create();
    Scene1.spawn_spikes();
    let mut player = Player::create(
        76.6,70.2,-0.1,
        &["Assets/textures/zombie.png"]);
    player.object.x = -400.0;
    


    let mut current = 0;

    
    
    
    let mut getkey = false;

    while !App.window_should_close("hello"){
        
        App.update_events();
        let delta_time = App.delta_time("hello");
        if App.key_is_pressed("hello",rmiv_engine::Key::Space) 
        {
            getkey = true;
        }
        
        if getkey{Scene1.move_all(&delta_time);}

        player.moved(&mut App,&delta_time,"hello");
        
        Scene1.move_spikes();

        

        App.begin_drawing("hello");
        
        backround.draw();
        Scene1.draw();

        player.draw();

        App.end_drawing("hello");
        
        

        
        

        

    }



}