use std::ffi::CString;




use crate::managers::platform::Platform;
use crate::{renderer::Mesh};
use crate::prelude::*;
 

pub struct Renderer
{
    texture_name:CString,
    offset_name:CString,
    scale_name:CString,
    rotation_name:CString,
    
}


pub fn deafultmesh() -> Mesh
{
    let vertices: [f32; 20] = [
            0.5,   -0.5,    0.5,  0.0, 0.0,
           -0.5,   -0.5,    0.5,  1.0, 0.0,
           -0.5,  0.5,    0.5,  1.0, 1.0,
            0.5,  0.5,    0.5,  0.0, 1.0,
        ];
        let indices: [u32; 6]= 
        [
            0,  1,  2,
            0,  2,  3,
        ];
    Mesh::gen_rectangle(vertices, indices)
}
impl Renderer
{
    pub fn create()-> Renderer
    {
        //binds default mesh to be used and sent to the gpu
         
        Renderer{
            texture_name:CString::new("texture1").unwrap(),
            offset_name:CString::new("offset").unwrap(),
            scale_name:CString::new("scale").unwrap(),
            rotation_name:CString::new("rotation").unwrap(),
        }
        
    }
    

    pub fn queue_draw(renderer:&mut Renderer,world:&mut World,platform:&mut Platform,assetmanager:&mut AssetManager)
    {
        let entities = world.get_render_components(world.query(crate::managers::world::ComponentType::Render));
        let cameras = world.get_camera_components(world.query(crate::managers::world::ComponentType::Camera));
        for i in 0..cameras.len()
        {
            let mesh = platform.windows.get_mesh(&cameras[i].targetwindow);
            let mesh = match mesh{
                Some(mesh)=>mesh,
                _=>{continue},
            };
            mesh.bind();
            let window = platform.windows.get_mut(&cameras[i].targetwindow);
            
            let window = match window{
                Some(wind)=>wind,
                _=>{continue},
            };
            
            
            window.begin_drawing();
            
            for j in 0..entities.len()
            {
                if entities[j].0.renderlayers.0 <= cameras[i].renderlayers.1 && cameras[i].renderlayers.0 <= entities[j].0.renderlayers.1 
                {
                    if entities[j].0.texture_id == None
                    {
                        println!("entity has no texture id");
                        continue;
                    }

                    let shader = assetmanager.shaders.get(&entities[j].0.shader_id);
                    let shader = match shader{
                    Some(sh) => sh,
                        _ => panic!("failed to get shader")
                    };
                    let texture = assetmanager.textures.get(&entities[j].0.texture_id.unwrap());
        
        
                    let texture = match texture{
                    Some(sh) => sh,
                        _ => panic!("no texture matches the id")
                    };
        
                    unsafe {
                    //binds textures shaders meshes and send the entity data to the gpu
                        shader.use_program();
          
                        let roation_location = gl::GetUniformLocation(shader.0,renderer.rotation_name.as_ptr());
                        let offset_location = gl::GetUniformLocation(shader.0,renderer.offset_name.as_ptr());
                        let scale_location = gl::GetUniformLocation(shader.0,renderer.scale_name.as_ptr());
                        let texture_location = gl::GetUniformLocation(shader.0,renderer.texture_name.as_ptr());
                        gl::Uniform1f(roation_location, entities[j].1.rotation);
                        gl::Uniform1i(texture_location,0);
                        gl::ActiveTexture(gl::TEXTURE0);
                        gl::BindTexture(gl::TEXTURE_2D, texture.0);
                        gl::Uniform3f(offset_location, entities[j].1.position.x,entities[j].1.position.y,entities[j].1.position.z);
                        gl::Uniform3f(scale_location, entities[j].1.scale.x,entities[j].1.scale.y,entities[j].1.scale.z);
                      
                        gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
                  
                    }
                }
            }
            window.end_drawing();

        }

    
    }
}