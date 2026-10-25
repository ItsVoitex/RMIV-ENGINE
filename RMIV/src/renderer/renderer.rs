use std::{ffi::CString};





pub use crate::backend::*;
 

pub struct Renderer
{
    texture_name:CString,
    offset_name:CString,
    scale_name:CString,
    rotation_name:CString,
    hastexture_name:CString,
    model_name:CString,
    view_name:CString,
    projection_name:CString,
    defaultshader:Shader,
    
}


pub fn deafultmesh() -> MeshInstance
{
    let vertices:Vec<Vec3> = vec![
            vec3(0.5,   -0.5,    0.5), 
           vec3(-0.5,   -0.5,    0.5),  
           vec3(-0.5,  0.5,    0.5),  
            vec3(0.5,  0.5,    0.5),  
        ];

    let indices:Vec<u32> = vec![ 
        
            0,  1,  2,
            0,  2,  3,
    ];
    
    
    MeshInstance::gendata(&Mesh { vertices: vertices, indices, color: None, texture_cords:None })
}
impl Renderer
{
    pub fn create()-> Renderer
    {
         
        Renderer{
            texture_name:CString::new("texture1").unwrap(),
            offset_name:CString::new("offset").unwrap(),
            scale_name:CString::new("scale").unwrap(),
            rotation_name:CString::new("rotation").unwrap(),
            view_name:CString::new("view").unwrap(),
            model_name:CString::new("model").unwrap(),
            projection_name:CString::new("proj").unwrap(),
            hastexture_name:CString::new("has_texture").unwrap(),
            defaultshader:Shader::create_default(),
        }
        
    }
    

    pub fn queue_draw(renderer:&mut Renderer,world:&mut World,platform:&mut Platform,assetmanager:&mut AssetManager)
    {
        let entities = world.get_render_components(world.query(crate::managers::world::ComponentType::Render));
        let cameras = world.get_camera_components(world.query(crate::managers::world::ComponentType::Camera));
        for i in 0..cameras.len()
        {
    
            
            let window = platform.windowmanager.get_mut(&cameras[i].targetwindow,false);
            
            let window = match window{
                Ok(wind)=>wind,
                _=>{continue},
            };
            window.make_current();
            window.begin_drawing();

             
            for j in 0..entities.len()
            {
                
                if entities[j].0.renderlayers.0 <= cameras[i].renderlayers.1 && cameras[i].renderlayers.0 <= entities[j].0.renderlayers.1 
                {
                    if  entities[j].1.position.x > ( 960.0 + entities[j].1.scale.x/2.0) || 
                        entities[j].1.position.x < (-960.0 - entities[j].1.scale.x/2.0) || 
                        entities[j].1.position.y < (-540.0 - entities[j].1.scale.y/2.0) || 
                        entities[j].1.position.y > ( 540.0 + entities[j].1.scale.y/2.0)
                    {
                        continue;
                    }
                    let mesh = assetmanager.meshes.get_asset(&entities[j].0.mesh_id).unwrap();
                    
                    unsafe {
                        gl::BindVertexArray(mesh.vao);
                    } 
                    let vericeslen = assetmanager.meshes.get_assetdata(&entities[j].0.mesh_id).unwrap().indices.len() as i32;
                    if entities[j].0.texture_id.is_some()
                    {
                        draw(renderer, Some(assetmanager.textures.get_asset(&entities[j].0.texture_id.unwrap())).unwrap(), entities[j],cameras[i],vericeslen);
                    }
                    else {
                        draw(renderer, None, entities[j],cameras[i],vericeslen);
                    }
                    
                   
                }
            }
            window.end_drawing();

        }

    
    }
}

fn draw(renderer:&mut Renderer,texture:Option<&TextureInstance>,entity:(&Render,&Transform),camera:&Camera,verticeslen:i32)
{
    unsafe {
        let shader = &renderer.defaultshader;     
        shader.use_program();
          
        let roation_location = gl::GetUniformLocation(shader.0,renderer.rotation_name.as_ptr());
        let offset_location = gl::GetUniformLocation(shader.0,renderer.offset_name.as_ptr());
        let scale_location = gl::GetUniformLocation(shader.0,renderer.scale_name.as_ptr());
        let has_texture_loation = gl::GetUniformLocation(shader.0, renderer.hastexture_name.as_ptr());
        let model_location = gl::GetUniformLocation(shader.0, renderer.model_name.as_ptr());
        let view_location = gl::GetUniformLocation(shader.0, renderer.view_name.as_ptr());
        let proj_location = gl::GetUniformLocation(shader.0, renderer.projection_name.as_ptr());
        match texture {
            Some(texture) => 
            {
                let texture_location = gl::GetUniformLocation(shader.0,renderer.texture_name.as_ptr());
                gl::Uniform1i(has_texture_loation, 1);
                gl::Uniform1i(texture_location,0);
                gl::Uniform3f(roation_location, entity.1.rotation.x,entity.1.rotation.y,entity.1.rotation.z);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture.0);
                gl::Uniform3f(offset_location, entity.1.position.x,entity.1.position.y,entity.1.position.z);
                gl::Uniform3f(scale_location, entity.1.scale.x,entity.1.scale.y,entity.1.scale.z);

                gl::UniformMatrix4fv(model_location,1,gl::FALSE,camera.model.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(view_location,1,gl::FALSE,camera.view.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(proj_location,1,gl::FALSE,camera.proj.to_cols_array().as_ptr());
                

                gl::DrawElements(gl::TRIANGLES, verticeslen,gl::UNSIGNED_INT,std::ptr::null());
            }
            None => {
                gl::Uniform1i(has_texture_loation, 0);
                gl::Uniform3f(roation_location, entity.1.rotation.x,entity.1.rotation.y,entity.1.rotation.z);
                gl::Uniform3f(offset_location, entity.1.position.x,entity.1.position.y,entity.1.position.z);
                gl::Uniform3f(scale_location, entity.1.scale.x,entity.1.scale.y,entity.1.scale.z);
        
                gl::DrawElements(gl::TRIANGLES, verticeslen,gl::UNSIGNED_INT,std::ptr::null());            
            }
        }
        
        
                  
    }
}

/*
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
                  
                    } */