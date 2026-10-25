
mod player;
mod scene;
use scene::Scene;
use player::Player;
use rmiv_engine::{backend::Mesh, managers::{asset::Texture, window}, prelude::*};
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
    speed:f32,
    player:Option<Player>,
    title:Entity,
    choice:Entity,
    over:Entity,
    state:i32,
    getkey:bool,
    spike_id:AssetHandle,
    scene:Option<Scene>,
    diff:Entity,
    mesh:AssetHandle


}
fn startup(assets:&mut AssetManager,world:&mut World,platform:&mut Platform) ->Game
{
    
    let diff_id = assets.load(Texture{path:"Assets/textures/difficulty.png".to_string()});

    let window = platform.create_window(Window{
        title:"zombiedash".to_string(),
        dimensions:(1920, 1080),
        is_primary:false,
        windowref:None
    });
    let defaultmesh =  assets.load(Mesh::rectangle(1.0, 1.0,1.0));
    let cube = assets.load(Mesh::cube(1.0, 1.0));
    let pyramid = assets.load(Mesh::pyramid(1.0, 1.0));
    world.spawn( (
    Render{
        texture_id:Some(assets.load(Texture{path:"Assets/textures/backround.png".to_string()})),
        shader_id:None,
        mesh_id:cube,
        renderlayers:(0,1)
    }, Transform{
        scale:vec3(1920.0, 1080.0, -1.0),
        position:vec3(0.0, 0.0, -2.0),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));

    let diff = world.spawn((Render{
        texture_id:Some(diff_id),
        mesh_id:defaultmesh,
        shader_id:None,
        renderlayers:(0,1)
    }, 
    Transform{
        scale:vec3(1920.0,500.0,-0.6,),
        position:vec3(0.0, 0.0, 0.0),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));


    let player = Player::create(
        &assets.load(Texture{path:"Assets/textures/zombie.png".to_string()}),
        96.0, 84.0, -0.1,
        world,&cube
    );
   
    let transform = world.get_transfom(&player.entity);
    transform.position.x = -400.0;
    transform.position.y = -500.0;
    
    
    

    let title_id = assets.load(Texture{path:"Assets/textures/ZombieDash.png".to_string()});
    let title = world.spawn((Render{
        texture_id:Some(title_id),
        mesh_id:defaultmesh,
        shader_id:None,
        renderlayers:(0,1)
    }, 
    Transform{
        scale:vec3(1200.0, 700.0, -0.6),
        position:vec3(0.0, 0.0, 0.0),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));

    
    let choice_id = assets.load(Texture{path:"Assets/textures/playagain.png".to_string()});
    let choice = world.spawn((Render{
        texture_id:Some(choice_id),
        mesh_id:defaultmesh,
        shader_id:None,
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(1920.0,300.0,-0.6,),
        position:vec3(0.0, 0.0, 0.0),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));
        
        
    
    let over_id = assets.load(Texture{path:"Assets/textures/gameover.png".to_string()});
    let over = world.spawn((Render{
        texture_id:Some(over_id),
        mesh_id:defaultmesh,
        shader_id:None,
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(1920.0,500.0,-0.6,),
        position:vec3(0.0, 0.0, 0.0),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));
    
    
    
    
    let mut scene1 = Scene::create();
    scene1.checkleveldat();
    



    world.spawn(Camera {
        position: vec3(0.0, 0.0, -2.0),
        rotation:  vec3(0.0,0.0,0.0), 
        renderlayers: (1, 1),
        targetwindow: window.clone(),
        ..Camera::camera2d()
    });
    
    
    let speed = 800.0;

      Game { 
        window, 
        speed,
         player: Some(player),
          title, choice, over, 
          state: 5, 
          getkey: false ,
          spike_id:assets.load(Texture{path:"Assets/textures/spike.png".to_string()}),
          scene:Some(scene1),
          diff,
          mesh:pyramid
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
    let player = match &mut game.player {
        Some(player) => player,
        None =>return
        
    };
    let scene1 = match &mut game.scene {
        Some(scene1) => scene1,
        None =>return
        
    };
    if game.state == 5
    {   scene1.checkleveldat();
        world.get_render(&game.title).renderlayers = (0,0);
        let diff = world.get_render(&game.diff);
        diff.renderlayers = (1,1);

        if platform.Key_is_pressed(&game.window, KeyCode::Num1)
        {
            game.speed = 800.0;
            diff.renderlayers = (0,0);
            world.get_render(&game.title).renderlayers= (1,1);
            game.state = 0;
        }
        else {if platform.Key_is_pressed(&game.window, KeyCode::Num2)
        {
            game.speed = 1200.0;
            diff.renderlayers = (0,0);
            world.get_render(&game.title).renderlayers= (1,1);
            game.state = 0;
        }
        else{if platform.Key_is_pressed(&game.window, KeyCode::Num3)
        {
            game.speed = 1500.0;
            diff.renderlayers = (0,0);
            world.get_render(&game.title).renderlayers = (1,1);
            game.state = 0;
        }}}
        return;
    }
    
    if game.state == 3
    {
        platform.close_window(game.window);
        return;
    }
    if game.state == 0
    {
        if platform.Key_is_pressed(&game.window,KeyCode::Space) 
        {
            game.getkey = true;
        }
        let delta_time = platform.delta_time(&game.window);
            
        
        if game.getkey{scene1.move_all(&delta_time,game.speed,world);}
        
        if game.getkey{player.moved(platform,world.get_transfom(&player.entity),&delta_time,&game.window,)};
        
        
        scene1.spawn_spikes(game.spike_id,world,&game.mesh);

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
        if platform.Key_is_pressed(&game.window, KeyCode::H)
        {
            temp.renderlayers = (5,5);
            game.state = 2;
        }
    }
    if game.state == 2 
    {
        let temp = world.get_render(&game.choice);
        temp.renderlayers = (1,1);
        if platform.Key_is_pressed(&game.window, KeyCode::Y)
        {
            temp.renderlayers = (5,5);
            scene1.checkleveldat();
            scene1.reset(world);
            game.getkey = false;
            let temp = world.get_transfom(&player.entity);
            temp.position.x = -400.0;
            temp.position.y = -500.0;
            temp.rotation =  vec3(0.0,0.0,0.0);
            player.rotationstate =  vec3(0.0,0.0,0.0);
            game.state = 5;
        }
        if platform.Key_is_pressed(&game.window, KeyCode::N)
        {
            game.state = 3;
        }
    }
}
