
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
    window:WindowHandle,
    speed:f32,
    player:Option<Player>,
    title:Entity,
    choice:Entity,
    state:i32,
    getkey:bool,
    spike_id:AssetHandle,
    scene:Option<Scene>,
    diff:Entity,
    mesh:AssetHandle,
    floor:f32,


}
fn startup(assets:&mut AssetManager,world:&mut World,platform:&mut Platform) ->Game
{
    
    let window = platform.create_window(Window{
        title:"zombiedash".to_string(),
        dimensions:(1920, 1080),
        targetfps:200,
        is_primary:false,
        windowref:None
    });
    let shader = assets.load(CustomShader::defaultshader());
    let defaultmesh =  assets.load(Mesh::rectangle(1.0, 1.0,1.0));
    let cube = assets.load(Mesh::cube(1.0, 1.0));
    let pyramid = assets.load(Mesh::square_base_pyramid(1.0, 1.0));
    



    let back = world.spawn( (
    Render{
        texture_id:Some(assets.load(Texture{path:"Assets/textures/backround.png".to_string()})),
        shader_id:Some(shader),
        mesh_id:defaultmesh,
        renderlayers:(0,1)
    }, Transform{
        scale:vec3(5.0, 3.0, 0.0),
        position:vec3(0.0, 0.1, -0.9),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));

    let back2 = world.spawn( (
    Render{
        texture_id:Some(assets.load(Texture{path:"Assets/textures/backround2.png".to_string()})),
        shader_id:Some(shader),
        mesh_id:defaultmesh,
        renderlayers:(0,1)
    }, Transform{
        scale:vec3(6.0, 3.0, 0.0),
        position:vec3(0.0, 0.0, -0.8),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));
    let back1 = world.spawn( (
    Render{
        texture_id:Some(assets.load(Texture{path:"Assets/textures/backround2.png".to_string()})),
        shader_id:Some(shader),
        mesh_id:defaultmesh,
        renderlayers:(0,1)
    }, Transform{
        scale:vec3(6.0, 3.0, 0.0),
        position:vec3(5.4, 0.0, -0.8),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));

    
  
    //
    let diff = world.spawn((Render{
        texture_id:Some(assets.load(Texture{path:"Assets/textures/difficulty.png".to_string()})),
        mesh_id:defaultmesh,
        shader_id:Some(shader),
        renderlayers:(0,1)
    }, 
    Transform{
        scale:vec3(4.0,1.0,0.0,),
        position:vec3(0.0, 0.0, -0.6),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));

    let floor:f32 = -1.03;
    let player = Player::create(
        &assets.load(Texture{path:"Assets/textures/zombie.png".to_string()}),
       0.23 , 0.21, 0.23,
        world,&cube,floor 
    );


    
    
    

    let title_id = assets.load(Texture{path:"Assets/textures/ZombieDash.png".to_string()});
    let title = world.spawn((Render{
        texture_id:Some(title_id),
        mesh_id:defaultmesh,
        shader_id:Some(shader),
        renderlayers:(0,1)
    }, 
    Transform{
        scale:vec3(2.8,1.5,0.0,),
        position:vec3(0.0, 0.0, -0.6),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));

    
    let choice_id = assets.load(Texture{path:"Assets/textures/playagain.png".to_string()});
    let choice = world.spawn((Render{
        texture_id:Some(choice_id),
        mesh_id:defaultmesh,
        shader_id:Some(shader),
        renderlayers:(0,0)
    }, 
    Transform{
        scale:vec3(4.0,0.8,0.0),
        position:vec3(0.0, 0.0, -0.6),
        rotation: vec3(0.0,0.0,0.0), 
    }
    ));
        
        
    
    
    
    
    
    let mut scene1 = Scene::create(floor,(back,back1,back2));
    scene1.checkleveldat();
    



    world.spawn(Camera {
        position: vec3(-0.0, 0.0, -1.5),
        rotation:  vec3(0.0,0.0,0.0), 
        renderlayers: (1, 1),
        fov:60.0,
        targetwindow: window.clone(),
        ..Camera::camera3d()
    });
    
    
    
    let speed = 800.0;

      Game { 
        window, 
        speed,
         player: Some(player),
          title, choice, 
          state: 5, 
          getkey: false ,
          spike_id:assets.load(Texture{path:"Assets/textures/spike.png".to_string()}),
          scene:Some(scene1),
          diff,
          mesh:pyramid,
          floor
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
    if game.state == 3
    {
        platform.close_window(game.window);
        return;
    }
    if game.state == 5
    {   scene1.checkleveldat();
        world.get_render(&game.title).renderlayers = (0,0);
        let diff = world.get_render(&game.diff);
        diff.renderlayers = (1,1);

        if platform.Key_is_pressed(&game.window, KeyCode::Num1)
        {
            game.speed = 2.0;
            diff.renderlayers = (0,0);
            world.get_render(&game.title).renderlayers= (1,1);
            game.state = 0;
        }
        else {if platform.Key_is_pressed(&game.window, KeyCode::Num2)
        {
            game.speed = 3.0;
            diff.renderlayers = (0,0);
            world.get_render(&game.title).renderlayers= (1,1);
            game.state = 0;
        }
        else{if platform.Key_is_pressed(&game.window, KeyCode::Num3)
        {
            game.speed = 4.0;
            diff.renderlayers = (0,0);
            world.get_render(&game.title).renderlayers = (1,1);
            game.state = 0;
        }}}
        return;
    }
    
    
    if game.state == 0
    {
        
        
        let delta_time = platform.delta_time(&game.window);
            
        
        if game.getkey{
            scene1.move_all(&delta_time,game.speed,world);scene1.spawn_spikes(game.spike_id,world,&game.mesh);
            player.moved(platform,world.get_transfom(&player.entity),&delta_time,&game.window,);
            game.speed += 0.0001;
        }
        
       
        
        

        let temp = world.get_render(&game.title);
        if !game.getkey {
            temp.renderlayers = (1,1)
        }
        else {
            temp.renderlayers = (5,5)
        }

        
        if platform.Key_is_pressed(&game.window,KeyCode::Space) 
        {
            game.getkey = true;
        }
        if world.check_collision(&player.entity, &scene1.getobjects(), false)
        {
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
            temp.position.x = -1.25;
            temp.position.y = game.floor;
            temp.rotation =  vec3(0.0,0.0,0.0);
            player.rotationstate =  vec3(0.0,0.0,0.0);
            game.state = 5;
        }
        
        
        
        if platform.Key_is_pressed(&game.window, KeyCode::N)
        {
            game.state = 3;
            return;
        }
    }
}
