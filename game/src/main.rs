
mod player;
mod scene;
use scene::Scene;
use player::Player;
use rmiv_engine::prelude::*;
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

struct GameState
{
    player:Player,
    scene1:Scene,
    title:Object,
    getkey:bool,
    backround:Object,
    spikeid:u32,

}

fn start(app:&mut App<GameState>) -> GameState
{
    let width = 2560;
    let height = 1440;
    
    app.create_window(width, height, "Zombie Dash");
    let default = app.load_default_shader();
        
    let backround = Object::create(
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
    
    let mut title = Object::create(textured_mesh,1000.0, 600.0, -0.6,default);
    let mut getkey = false;
    scene1.checkleveldat();

    /*while !app.window_should_close(0)
    {
        let delta_time = app.delta_time(0);
        app.update_events();
        if app.key_is_pressed(0,Key::Space) 
        {
            getkey = true;
        }
        
        if getkey{scene1.move_all(&delta_time);}
        
        player.moved(app,&delta_time,0);
        
        
        scene1.spawn_spikes(spike);
        


        app.begin_drawing(0);
        app.queue_draw( vec![&backround,&player.object],0);
        scene1.draw(app,0);

        if !getkey{app.draw(&mut title,0);};

        app.end_drawing(0);
    
        
    }
    */
    GameState { player, scene1, title, getkey, backround, spikeid:spike  }
}
fn main()
{
    App::<GameState>::new()
    .add_startup_system(start)
    .add_update_system(update_events)
    .run();

}

fn update_events(app:&mut App<GameState>,state:&mut GameState)
{
    let delta_time = app.delta_time(0);
    app.update_events();
    if app.key_is_pressed(0,Key::Space) 
    {
        state.getkey = true;
    }
        
    if state.getkey{state.scene1.move_all(&delta_time);}
        
    state.player.moved(app,&delta_time,0);
        
        
    state.scene1.spawn_spikes(state.spikeid);
        


    app.begin_drawing(0);
    app.queue_draw( vec![&state.backround,&state.player.object],0);
    state.scene1.draw(app,0);

    if !state.getkey{app.draw(&mut state.title,0);};

    app.end_drawing(0);
}
