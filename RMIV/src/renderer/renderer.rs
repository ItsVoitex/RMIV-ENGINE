use std::{ffi::CString};





pub use crate::backend::*;
use crate::managers::{window::WindowManager, world::Color};
 

pub struct Renderer
{
    texture_name:CString,
    offset_name:CString,
    scale_name:CString,
    rotation_name:CString,
    hastexture_name:CString,
    model_name:CString,
    view_name:CString,
    color_name:CString,
    projection_name:CString,
    defaultshader:ShaderInstance,
    defaultmesh:Option<MeshInstance>,
    pub rendersnapshot:Vec<(Render,Transform,Option<Color>)>,
    pub cameras:Vec<Camera>
    
    
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
    pub fn create(enabledebug:bool)-> Renderer
    {
        let mut defaultmesh = None;
        
        if enabledebug
        {
            defaultmesh = Some(MeshInstance::gendata(&Mesh {
                
                color:Some(vec![
                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),

                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),

                ]),
                ..Mesh::texturelesscube(1.0, 1.0)}));
        }
        Renderer{
            texture_name:CString::new("texture1").unwrap(),
            offset_name:CString::new("offset").unwrap(),
            scale_name:CString::new("scale").unwrap(),
            rotation_name:CString::new("rotation").unwrap(),
            view_name:CString::new("view").unwrap(),
            model_name:CString::new("model").unwrap(),
            projection_name:CString::new("proj").unwrap(),
            color_name:CString::new("Color").unwrap(),
            hastexture_name:CString::new("has_texture").unwrap(),
            defaultshader:ShaderInstance::create_default(),
            defaultmesh,
            rendersnapshot:Vec::new(),
            cameras:Vec::new()
        }
        
    }
    

    pub fn queue_draw(&mut self,platform:&mut WindowManager,assetmanager:&mut AssetManager,world:&mut World)
    {
      
        
        let mut hitboxes:Option<Vec<(&Render, &Transform, &Hitbox)>> = None;
        if let Some(_) = &self.defaultmesh
        {
            hitboxes = Some(world.get_render_hitboxes())
        }
        for i in 0..self.cameras.len()
        {
            let window = platform.get_mut(&self.cameras[i].targetwindow,false);
            
            let window = match window{
                Ok(wind)=>wind,
                _=>{continue},
            };
            if !window.frameinfo.shouldrender
            {
                continue;
            }
            window.make_current();
            window.begin_drawing();

             
            for j in 0..self.rendersnapshot.len()
            {
                
                if self.rendersnapshot[j].0.renderlayers.0 <= self.cameras[i].renderlayers.1 && self.cameras[i].renderlayers.0 <= self.rendersnapshot[j].0.renderlayers.1 
                {
                    /*if  entities[j].1.position.x > ( 960.0 + entities[j].1.scale.x/2.0)  || 
                        entities[j].1.position.x < (-960.0- entities[j].1.scale.x/2.0) || 
                        entities[j].1.position.y < (-540.0 - entities[j].1.scale.y/2.0) || 
                        entities[j].1.position.y > ( 540.0 + entities[j].1.scale.y/2.0)
                    {
                        continue;
                    }
                    */
                    
                    let mesh = assetmanager.meshes.get_instance(&self.rendersnapshot[j].0.mesh_id,true).unwrap();
                    
                    unsafe {
                        gl::BindVertexArray(mesh.vao);
                    } 
                    
                    
                    let vericeslen = assetmanager.meshes.get_data(&self.rendersnapshot[j].0.mesh_id,true).unwrap().indices.len() as i32;
                    if let Some(texture_id) = &self.rendersnapshot[j].0.texture_id
                    {
                        if let Some(_) = &self.rendersnapshot[j].2
                        {
                            draw(self, Some(assetmanager.textures.get_instance(texture_id,true).unwrap()), j,i,vericeslen,Some(j));
                        }
                        else {
                            draw(self, Some(assetmanager.textures.get_instance(texture_id,true).unwrap()), j,i,vericeslen,None);
                        }
                        
                    }
                    else {
                        if let Some(_) = &self.rendersnapshot[j].2
                        {
                            draw(self, None, j,i,vericeslen,Some(j));
                        }
                        else {
                            draw(self, None,j,i,vericeslen,None);
                        }
        
                    }

                    
                    
                }

            }
            if let Some(entity) = &hitboxes
            {
                for k in entity.iter()
                {
                    drawhitbox(self, *k,i);
                } 
                        
            }
            window.frameinfo.fps +=1;
            window.end_drawing();

        }

    
    }
     
}

fn draw(renderer:&mut Renderer,texture:Option<&TextureInstance>,entity:usize,camera:usize,verticeslen:i32,color:Option<usize>)
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
        let color_location = gl::GetUniformLocation(shader.0, renderer.color_name.as_ptr());
        match texture {
            Some(texture) => 
            {
                let texture_location = gl::GetUniformLocation(shader.0,renderer.texture_name.as_ptr());
                gl::Uniform1i(has_texture_loation, 1);
                gl::Uniform1i(texture_location,0);
                gl::Uniform3f(roation_location, renderer.rendersnapshot[entity].1.rotation.x,renderer.rendersnapshot[entity].1.rotation.y,renderer.rendersnapshot[entity].1.rotation.z);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture.0);
                gl::Uniform3f(offset_location, renderer.rendersnapshot[entity].1.position.x,renderer.rendersnapshot[entity].1.position.y,renderer.rendersnapshot[entity].1.position.z);
                gl::Uniform3f(scale_location, renderer.rendersnapshot[entity].1.scale.x,renderer.rendersnapshot[entity].1.scale.y,renderer.rendersnapshot[entity].1.scale.z);
                if let Some(colorindex) = color
                {
                    let color = &renderer.rendersnapshot[colorindex].2.unwrap();
                    gl::Uniform3f(color_location, color.red as f32/255.0,color.green as f32/255.0,color.blue as f32/255.0);
                }
                else {
                    let color = &Color{red:255,green:255,blue:255};
                    gl::Uniform3f(color_location, color.red as f32/255.0,color.green as f32/255.0,color.blue as f32/255.0);
                }
                
                gl::UniformMatrix4fv(model_location,1,gl::FALSE,renderer.cameras[camera].model.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(view_location,1,gl::FALSE,renderer.cameras[camera].view.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(proj_location,1,gl::FALSE,renderer.cameras[camera].proj.to_cols_array().as_ptr());
                

                gl::DrawElements(gl::TRIANGLES, verticeslen,gl::UNSIGNED_INT,std::ptr::null());
            }
            None => {
                gl::Uniform1i(has_texture_loation, 0);
                gl::Uniform3f(roation_location, renderer.rendersnapshot[entity].1.rotation.x,renderer.rendersnapshot[entity].1.rotation.y,renderer.rendersnapshot[entity].1.rotation.z);
                gl::Uniform3f(offset_location, renderer.rendersnapshot[entity].1.position.x,renderer.rendersnapshot[entity].1.position.y,renderer.rendersnapshot[entity].1.position.z);
                gl::Uniform3f(scale_location, renderer.rendersnapshot[entity].1.scale.x,renderer.rendersnapshot[entity].1.scale.y,renderer.rendersnapshot[entity].1.scale.z);
                if let Some(colorindex) = color
                {
                    let color = &renderer.rendersnapshot[colorindex].2.unwrap();
                    gl::Uniform3f(color_location, color.red as f32/255.0,color.green as f32/255.0,color.blue as f32/255.0);
                }
                else {
                    let color = &Color{red:255,green:255,blue:255};
                    gl::Uniform3f(color_location, color.red as f32/255.0,color.green as f32/255.0,color.blue as f32/255.0);
                }

                gl::UniformMatrix4fv(model_location,1,gl::FALSE,renderer.cameras[camera].model.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(view_location,1,gl::FALSE,renderer.cameras[camera].view.to_cols_array().as_ptr());
                gl::UniformMatrix4fv(proj_location,1,gl::FALSE,renderer.cameras[camera].proj.to_cols_array().as_ptr());
                

                gl::DrawElements(gl::TRIANGLES, verticeslen,gl::UNSIGNED_INT,std::ptr::null());            
            }
        }
        
        
                  
    }
}
fn drawhitbox(renderer:&mut Renderer,entity:(&Render,&Transform,&Hitbox),camera:usize)
{
    unsafe {

        if let Some(mesh) = &renderer.defaultmesh{
        let shader = &renderer.defaultshader;     
        shader.use_program();
     
        gl::BindVertexArray(mesh.vao);
       
          
        let roation_location = gl::GetUniformLocation(shader.0,renderer.rotation_name.as_ptr());
        let offset_location = gl::GetUniformLocation(shader.0,renderer.offset_name.as_ptr());
        let scale_location = gl::GetUniformLocation(shader.0,renderer.scale_name.as_ptr());
        let has_texture_loation = gl::GetUniformLocation(shader.0, renderer.hastexture_name.as_ptr());
        let model_location = gl::GetUniformLocation(shader.0, renderer.model_name.as_ptr());
        let view_location = gl::GetUniformLocation(shader.0, renderer.view_name.as_ptr());
        let proj_location = gl::GetUniformLocation(shader.0, renderer.projection_name.as_ptr());
        let color_location = gl::GetUniformLocation(shader.0, renderer.color_name.as_ptr());

        gl::Uniform1i(has_texture_loation, 0);
        gl::Uniform3f(roation_location, entity.1.rotation.x,entity.1.rotation.y,entity.1.rotation.z);
        gl::Uniform3f(offset_location, entity.1.position.x,entity.1.position.y,entity.1.position.z);
        gl::Uniform3f(scale_location, entity.2.length,entity.2.height,entity.2.width);
        gl::Uniform3f(color_location, 1.0,1.0,1.0);

        gl::UniformMatrix4fv(model_location,1,gl::FALSE,renderer.cameras[camera].model.to_cols_array().as_ptr());
        gl::UniformMatrix4fv(view_location,1,gl::FALSE,renderer.cameras[camera].view.to_cols_array().as_ptr());
        gl::UniformMatrix4fv(proj_location,1,gl::FALSE,renderer.cameras[camera].proj.to_cols_array().as_ptr());
                

        gl::DrawElements(gl::TRIANGLES, 36,gl::UNSIGNED_INT,std::ptr::null());  
        
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