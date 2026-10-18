
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

pub struct Game
{
    window1:WindowHandle,
    window2:WindowHandle,
    speed:f32,
    player:Option<Player>,
    title:Entity,
    choice:Entity,
    over:Entity,
    state:i32,
    getkey:bool,
    spike_id:u32,
    scene:Option<Scene>

}
fn startup2(assets:&mut AssetManager,world:&mut World,platform:&mut Platform,game:&mut Game) ->Game
{
    

    let player = Player::create(
        assets.load_texture_from_file("Assets/textures/zombie.png"),
        80.0*1.2, 70.0*1.2, -0.1,
        0
        ,world
    );
   
    let transform = world.get_transfom(&player.entity);
    transform.position.x = -400.0;
    transform.position.y = -500.0;
    
    
    

    let title_id = assets.load_texture_from_file("Assets/textures/ZombieDash.png");
    let title = world.spawn((Render{
        texture_id:Some(title_id),
        shader_id:0,
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(1200.0, 700.0, -0.6),
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0
    }
    ));

    
    let choice_id = assets.load_texture_from_file("Assets/textures/playagain.png");
    let choice = world.spawn((Render{
        texture_id:Some(choice_id),
        shader_id:0,
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(1920.0,300.0,-0.6,),
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0
    }
    ));
        
        
    
    let over_id = assets.load_texture_from_file("Assets/textures/gameover.png");
    let over = world.spawn((Render{
        texture_id:Some(over_id),
        shader_id:0,
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(1920.0,500.0,-0.6,),
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0
    }
    ));
    
    
    
    
    let mut scene1 = Scene::create();
    scene1.checkleveldat();
    let window2 = platform.create_window(1920, 1080, "zombiedash");



    world.spawn(Camera {
        position: vec3(0.0, 0.0, 0.0),
        rotation: 0.0,
        renderlayers: (1, 1),
        targetwindow: window2.clone(),
    });
    Game { 
        window1: game.window1,
        window2, 
        speed: game.speed,
         player: Some(player),
          title, choice, over, 
          state: 0, 
          getkey: false ,
          spike_id:assets.load_texture_from_file("Assets/textures/spike.png"),
          scene:Some(scene1)
    }
        
}
fn startup(assets:&mut AssetManager,world:&mut World,platform:&mut Platform) -> Game
{
    
    let window1 = platform.create_window(1920, 540, "Choose difficulty");
    let diff_id = assets.load_texture_from_file("Assets/textures/difficulty.png");

    world.spawn( (
    Render{
        texture_id:Some(assets.load_texture_from_file("Assets/textures/backround.png")),
        shader_id:0,
        renderlayers:(0,1)
    }, Transform{
        scale:vec3(1920.0, 1080.0, 1.0),
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0
    }
    ));
    world.spawn((Render{
        texture_id:Some(diff_id),
        shader_id:0,
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(1920.0,500.0,-0.6,),
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0
    }
    ));
    world.spawn(Camera{
        position:vec3(0.0, 0.0, 0.0),
        rotation:0.0,
        renderlayers:(0,0),
        targetwindow:window1.clone()
    });
    let speed = 800.0;
    Game { window1,window2:window1, speed ,player:None,title:0,choice:0,over:0,state:0,getkey:false,spike_id:0,scene:None}
}
fn update1(assets:&mut AssetManager,world:&mut World,platform:&mut Platform,game:&mut Game)
{
    if platform.Key_is_pressed(&game.window1, KeyCode::Num1)
    {
        game.speed = 800.0;
        *game = startup2(assets, world, platform,game);
        platform.close_window(game.window1);
    }
    if platform.Key_is_pressed(&game.window1, KeyCode::Num2)
    {
            game.speed = 1200.0;
            *game = startup2(assets, world, platform,game);
            platform.close_window(game.window1);
    }
    if platform.Key_is_pressed(&game.window1, KeyCode::Num3)
    {
            game.speed = 1500.0;
            *game = startup2(assets, world, platform,game);
            platform.close_window(game.window1);
    }
    
}      


fn main()
{
    
    App::new()
    .add_startupstate_system(startup)
    .add_updatestate_system(update1)
    .add_updatestate_system(update2)
    .run();

    
}
 
fn update2(_assets:&mut AssetManager,world:&mut World,platform:&mut Platform,game:&mut Game)
{
    let player = match &mut game.player {
        Some(player) => player,
        None =>return
        
    };
    let scene1 = match &mut game.scene {
        Some(scene1) => scene1,
        None =>return
        
    };
    if game.state == 3
    {
        return;
    }
    if game.state == 0
    {
        if platform.Key_is_pressed(&game.window2,KeyCode::Space) 
        {
            game.getkey = true;
        }
        let delta_time = platform.delta_time(&game.window2);
            
        
        if game.getkey{scene1.move_all(&delta_time,game.speed,world);}
        
        if game.getkey{player.moved(platform,world.get_transfom(&player.entity),&delta_time,&game.window2,)};
        
        
        scene1.spawn_spikes(game.spike_id,world);

        let temp = world.get_render(&game.title);
        if !game.getkey {
            temp.renderlayers = (1,1)
        }
        else {
            temp.renderlayers = (5,5)
        }

        
            
        if world.check_collision(&player.entity, &scene1.getobjects(), false)
        {
            game.state = 1;
        }

    }

    if game.state == 1 
    {
        let temp = world.get_render(&game.over);
        temp.renderlayers = (1,1);
        if platform.Key_is_pressed(&game.window2, KeyCode::H)
        {
            temp.renderlayers = (5,5);
            game.state = 2;
        }
    }
    if game.state == 2 
    {
        let temp = world.get_render(&game.choice);
        temp.renderlayers = (1,1);
        if platform.Key_is_pressed(&game.window2, KeyCode::Y)
        {
            temp.renderlayers = (5,5);
            scene1.checkleveldat();
            scene1.reset(world);
            game.getkey = false;
            let temp = world.get_transfom(&player.entity);
            temp.position.x = -400.0;
            temp.position.y = -500.0;
            game.state = 0;
        }
        if platform.Key_is_pressed(&game.window2, KeyCode::N)
        {
                
            game.state = 3;
        }
    }
}
