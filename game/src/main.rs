
mod player;
mod scene;
use scene::Scene;
use player::Player;
use rmiv_engine::{self, App};
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
//implement a manager of all objects then check if they have collisions if so then make them collide


fn start(app:&mut App)
{
    let width = 2560;
    let height = 1440;
    
    app.create_window(width, height, "Zombie Dash");
    let default = app.load_default_shader();
        
    let backround = rmiv_engine::Object::create(
        app.load_texture_from_file("Assets/textures/backround.png")
        ,1920.0, 1080.0, 1.0,0);
    
    let mut scene1 = Scene::create();
    let mut player = Player::create(
        app.load_texture_from_file("Assets/textures/zombie.png"),
        80.0*1.2, 70.0*1.2, -0.1,
        0
    );
    player.object.position.x = -400.0;

    let spike = app.load_texture_from_file("Assets/textures/spike.png");

    let textured_mesh = app.load_texture_from_file("Assets/textures/ZombieDash.png");
    
    let mut title = rmiv_engine::Object::create(textured_mesh,1000.0, 600.0, -0.6,default);
    let mut getkey = false;

    while !app.window_should_close(0)
    {
        let delta_time = app.delta_time(0);
        app.update_events();
        if app.key_is_pressed(0,rmiv_engine::Key::Space) 
        {
            getkey = true;
        }
        
        if getkey{scene1.move_all(&delta_time);}
        
        player.moved(app,&delta_time,0);
        
        
        scene1.spawn_spikes(spike);
        


        app.begin_drawing(0);
        app.queue_draw( vec![&backround,&player.object],0);
        scene1.draw(app,0);

        if !getkey{title.draw(app,0);};

        app.end_drawing(0);
    
    scene1.checkleveldat();
    }
}
fn main()
{
    
    App::new()
    .on_startup(start)
    .run();

}
