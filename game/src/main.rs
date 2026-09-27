mod player;
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

    let mut app = rmiv_engine::App::new();
    app.create_window(width, height, "Zombie Dash","hello");
    app.assets.load_shader();

    
    

    
        
    let mut backround = rmiv_engine::Object::create(
        &app.assets.load_texture_from_file("Assets/textures/backround.png","backround"),

        &app.assets.load_mesh(1920.0, 1080.0, 0.0, "backround"),

        "default"
    );

    

    let mut scene1 = scene::Scene::create();
    scene1.spawn_spikes(&mut app.assets);
    let mut player = Player::create(
        &app.assets.load_texture_from_file("Assets/textures/zombie.png","zombie"),
        &app.assets.load_mesh(76.6, 70.2, -0.1, "player"),
        "default"
    );
    player.object.position.x = -400.0;
    



    
    
    
    let mut getkey = false;

    while !app.window_should_close("hello"){
        
        app.update_events();
        let delta_time = app.delta_time("hello");
        if app.key_is_pressed("hello",rmiv_engine::Key::Space) 
        {
            getkey = true;
        }
        
        if getkey{scene1.move_all(&delta_time);}

        player.moved(&mut app,&delta_time,"hello");
        
        scene1.move_spikes();

        

        app.begin_drawing("hello");
        
        backround.draw(&mut app);
        scene1.draw(&mut app);

        player.draw(&mut app);

        app.end_drawing("hello");
        

    }



}