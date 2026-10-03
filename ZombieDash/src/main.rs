
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



fn main()
{
    let width = 1920;
    let height = 1080;
    
    let mut app = App::new();

    app.create_window(width, height, "Zombie Dash");
    

    
    let default = app.load_default_shader();
    

    
    let mut backround = Object::create(
        app.load_texture_from_file("Assets/textures/backround.png"),
        1920.0, 1080.0, 1.0,
        default
    );
    backround.set_hitbox(1920.0, 1080.0, 5.0);

    let mut player = Player::create(
        app.load_texture_from_file("Assets/textures/zombie.png"),
        80.0*1.2, 70.0*1.2, -0.1,
        default
    );
    player.object.set_hitbox(50.0, 50.0, 5.0);
    player.object.position.x = -400.0;
    player.object.position.y = -500.0;
    
    
    

    let title_id = app.load_texture_from_file("Assets/textures/ZombieDash.png");
    let mut title = Object::create(
        title_id,
        1200.0, 700.0, -0.6,
        default
    );
    
    let choice_id = app.assets.load_texture_from_file("Assets/textures/playagain?.png");
    let mut choice = Object::create(
        choice_id,
        1920.0,300.0,-0.6,
        default
    );
    
    let over_id = app.assets.load_texture_from_file("Assets/textures/gameover.png");
    let mut over = Object::create(
        over_id,
        1920.0,500.0,-0.6,
        default
    );
    
    
    let mut scene1 = Scene::create();
    let spike_id = app.load_texture_from_file("Assets/textures/spike.png");
    scene1.checkleveldat();
    
    let mut getkey = false;
    let mut game_state = 0;


    while !app.window_should_close(0)
    {
        if game_state == 3
        {
            break;
        }
        while game_state == 0 && !app.window_should_close(0)
        {
            if app.key_is_pressed(0,Key::Space) 
            {
                getkey = true;
            }
            app.update_events();
            let delta_time = app.delta_time(0);
            
        
            if getkey{scene1.move_all(&delta_time);}
        
            if getkey{player.moved(&mut app,&delta_time,0)};
        
        
            scene1.spawn_spikes(spike_id);
        
            app.begin_drawing(0);
        
            app.queue_draw( vec![&backround,&player.object],0);
            scene1.draw(&mut app,0);

            if !getkey{app.draw(&mut title,0);};

            app.end_drawing(0);
            

            if player.object.check_collision(&scene1.objects,false) {game_state = 1};


        }

        player.object.position.x = -400.0;
        player.object.position.y = -500.0;
        while game_state == 1 && !app.window_should_close(0)
        {
            app.update_events();
            app.begin_drawing(0);
            app.draw(&mut over, 0);
            app.end_drawing(0);
            if app.key_is_pressed(0, Key::H)
            {
                game_state = 2;
            }
        }
        while game_state == 2 && !app.window_should_close(0){
            player.object.position.x = -400.0;
            player.object.position.y = -500.0;
            app.update_events();
            app.begin_drawing(0);
            app.draw(&mut choice, 0);
            app.end_drawing(0);
            if app.key_is_pressed(0, Key::Y)
            {
                scene1.checkleveldat();
                scene1.reset();
                getkey = false;
                player.object.position.x = -400.0;
                player.object.position.y = -500.0;
                game_state = 0;
                break;
            }
            if app.key_is_pressed(0, Key::N)
            {
                
                game_state = 3;
            }
        }
    }
}
