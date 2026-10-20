
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
pub struct Game{
    player:Player,
    worldoffset:Vec3,
    window:WindowHandle
}


fn startup(assets:&mut AssetManager,world:&mut World,platform:&mut Platform) -> Game
{
    
    let mut scene1 = Scene::create();
    scene1.checkleveldat();
    let window = platform.create_window(1920, 1080, "zombiedash");



    world.spawn(Camera {
        position: vec3(0.0, 0.0, 0.0),
        rotation: 0.0,
        renderlayers: (1, 1),
        targetwindow: window.clone(),
    });


    
    let backround = world.spawn( (
    Render{
        texture_id:Some(assets.load_texture_from_file("Assets/textures/backround.png")),
        shader_id:0,
        renderlayers:(0,1)
    }, Transform{
        scale:vec3(1920.0, 1080.0, 1.0),
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0
    },));
    
    let backround2 = world.clone(&backround);
     let player = Player::create(
        assets.load_texture_from_file("Assets/textures/zombie.png"),
        80.0*1.2, 70.0*1.2, -0.1,
        0
        ,world
    );
    let worldoffset= vec3(0.0,0.0,0.0);
    Game { player,worldoffset ,window}
}



fn main()
{
    
    App::new()
    .add_startupstate_system(startup)
    .add_updatestate_system(update)
    .run();

    
}

fn update(assets:&mut AssetManager,world:&mut World,platform:&mut Platform,game:&mut Game)
{
    let deltatime = platform.delta_time(&game.window);
    let mut x = world.query(rmiv_engine::managers::world::ComponentType::Transform);
    for i in 0..x.len()
    {
        if x[i] == game.player.entity
        {
            x.remove(i);
        }
    }
  
    if game.player.moved(platform , &deltatime,&game.window ,&mut game.worldoffset)
    {
        for i in x.iter()
        {
           world.get_transfom(&i).position.x = game.worldoffset.x;
            world.get_transfom(&i).position.y = game.worldoffset.y; 
        }
    }

    

}

