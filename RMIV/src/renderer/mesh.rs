use gl;
use glam::{Vec2, Vec3};
use std:: ffi::c_void;
use crate::prelude::*;
pub struct MeshInstance{
    pub vbo:u32,
    pub vao:u32,
    pub ebo:u32,
}

//generates vbo which store the verticies vao for how the vertices are layed out and ebo to generate meshes without duplicate verictes as some overlap
impl MeshInstance{
    pub fn gendata(mesh:&Mesh) -> MeshInstance
    {
        let texture_cords:Vec<f32> = vec![
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            ];

        let color:Vec<f32> = vec![ 
                1.0,1.0,1.0,
            ];
        let mut index = 0;
        let mut index2 = 0;
        
        
        let mut vertices = Vec::new();
        for i in 0..mesh.vertices.len()
        {
            vertices.push(mesh.vertices[i].x);
            vertices.push(mesh.vertices[i].y);
            vertices.push(mesh.vertices[i].z);
            match &mesh.texture_cords {
                Some(val) => {
                    if i<val.len()
                    {
                        vertices.push(val[i].x);
                        vertices.push(val[i].y);
                    }
                    else {
                        vertices.push(texture_cords[index]);
                        vertices.push(texture_cords[index+1]);
                        index+=2;
                        if index >= texture_cords.len()
                        {
                            index = 0;
                        }
                    }
                }
                None => {
                    
                    vertices.push(texture_cords[index]);
                    vertices.push(texture_cords[index+1]);
                    index+=2;
                    if index >= texture_cords.len()
                    {
                        index = 0;
                    }
                }, 
            }
            match &mesh.color {
                Some(val) => {
                    if i<val.len()
                    {
                        vertices.push(val[i].x);
                        vertices.push(val[i].y);
                        vertices.push(val[i].z);
                    }
                    else {
                        vertices.push(color[index2]);
                        vertices.push(color[index2+1]);
                        vertices.push(color[index2+2]);
                        if index2 >= color.len()
                        {
                            index2 = 0;
                        }
                    }
                }
                None => {
                    
                    vertices.push(color[index2]);
                    vertices.push(color[index2+1]);
                    vertices.push(color[index2+2]);
                    if index2 >= color.len()
                    {
                        index2 = 0;
                    }
                    
                }, 
            }

        }
        
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);


            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * vertices.len()) as isize,vertices.as_ptr() as *const c_void,gl::STATIC_DRAW);



            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (std::mem::size_of::<u32>() * mesh.indices.len()) as isize, mesh.indices.as_ptr() as *const c_void, gl::STATIC_DRAW);


            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,(std::mem::size_of::<f32>() * 8) as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE,(std::mem::size_of::<f32>() * 8) as i32,(std::mem::size_of::<f32>()*3) as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE,(std::mem::size_of::<f32>() * 8) as i32,(std::mem::size_of::<f32>()*5) as *const c_void);
            gl::EnableVertexAttribArray(2);

            gl::BindVertexArray(0);
            
        }
        
        

        MeshInstance{vao,vbo,ebo}
    }


    


    pub fn bind(&self)
    {
        unsafe{
            gl::BindVertexArray(self.vao);
        }
    }
}


impl Drop for MeshInstance
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteBuffers(1, &self.vbo);        
            gl::DeleteVertexArrays(1, &self.vao);
        }
    
    }
}
pub struct Mesh
{
    pub vertices:Vec<Vec3>,
    pub indices:Vec<u32>,
    pub color:Option<Vec<Vec3>>,
    pub texture_cords:Option<Vec<Vec2>>
}
impl Mesh
{
    pub fn rectangle(width:f32,height:f32,depth:f32) -> Mesh
    {
        let vertices: Vec<Vec3>  = vec![ 
            vec3(width / 2.0,  -height / 2.0,    depth),
            vec3(-width / 2.0,  -height / 2.0,    depth),
            vec3(-width / 2.0,   height / 2.0,    depth),
           vec3(width / 2.0,   height / 2.0,    depth),
        ];
        let texture_cords:Vec<Vec2> = vec![
            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,)
        ];
        let indices:Vec<u32>= vec!
        [
            0,  1,  2,
            0,  2,  3,
        ];
        Mesh { vertices, indices, color: None, texture_cords:Some(texture_cords) }
    }
    /*pub fn circle(diameter:f32,precision:f32,depth:f32) 
    {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let top = vec3(0.0, diameter/2.0,depth);
        let bottom = vec3(0.0, -diameter/2.0,depth);
        let left = vec3(diameter/2.0 ,0.0,depth);
        let left = vec3(-diameter/2.0 ,0.0,depth);
        

       


        //Mesh { vertices, indices, color: None, texture_cords:Some(texture_cords) }
    }
    */
    
    pub fn cube(width:f32,height:f32) -> Mesh
    {
         let vertices: Vec<Vec3>  = vec![ 
            vec3(width / 2.0,  -height / 2.0,   width/2.0), //front 0 on
            vec3(-width / 2.0,  -height / 2.0,  width/2.0),
            vec3(-width / 2.0,   height / 2.0,  width/2.0),
           vec3(width / 2.0,   height / 2.0,    width/2.0),

            vec3(width / 2.0,  -height / 2.0,    -width/2.0), //back 4 on
            vec3(-width / 2.0,  -height / 2.0,    -width/2.0),
            vec3(-width / 2.0,   height / 2.0,    -width/2.0),
           vec3(width / 2.0,   height / 2.0,    -width/2.0),

            vec3(width / 2.0,  -height / 2.0,    -width/2.0), //right 8 on
            vec3(width / 2.0,  -height / 2.0,   width/2.0),
            vec3(width / 2.0,   height / 2.0,    width/2.0),
            vec3(width / 2.0,   height / 2.0,    -width/2.0),

            vec3(-width / 2.0,  -height / 2.0,  width/2.0), //left 12 on
             vec3(-width / 2.0,  -height / 2.0,    -width/2.0),
             vec3(-width / 2.0,   height / 2.0,    -width/2.0),
             vec3(-width / 2.0,   height / 2.0,  width/2.0),

             vec3(width / 2.0,  -height / 2.0,    -width/2.0),//bottom 16 on
             vec3(-width / 2.0,  -height / 2.0,    -width/2.0),
             vec3(-width / 2.0,  -height / 2.0,  width/2.0),
             vec3(width / 2.0,  -height / 2.0,   width/2.0),

             vec3(width / 2.0,   height / 2.0,    width/2.0),//top 20 on
             vec3(-width / 2.0,   height / 2.0,  width/2.0),
             vec3(-width / 2.0,   height / 2.0,    -width/2.0),
             vec3(width / 2.0,   height / 2.0,    -width/2.0),

        ];
        let indices:Vec<u32>= vec!
        [
            0,  1,  2,
            0,  2,  3,

            4, 5, 6,
            4, 6, 7,

            8,9,10,
            8,10,11,

            12,13,14,
            12,14,15,

            16,17,18,
            16,18,19,

            20,21,22,
            20,22,23
        ];

        let texture_cords:Vec<Vec2> = vec![
            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,),

            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,),

            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,),

            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,),

            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,),

            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            vec2(0.0, 1.0,),
            vec2(1.0, 1.0,)
        ];
        Mesh { vertices, indices, color: None, texture_cords:Some(texture_cords) }
    }
    pub fn pyramid(width:f32,height:f32) -> Mesh
    {
         let vertices: Vec<Vec3>  = vec![ 
            vec3(width / 2.0,  -height / 2.0,    width/2.0),
            vec3(-width / 2.0,  -height / 2.0,    width/2.0),

            vec3(width / 2.0,  -height / 2.0,    -width/2.0),
            vec3(-width / 2.0,  -height / 2.0,    -width/2.0),

            vec3(0.0,  height / 2.0,    0.0),

        ];
        let indices:Vec<u32>= vec!
        [
            0,1,3,
            0,3,2,

            0,1,4,

            2,0,4,

            3,2,4,

            3,1,4
        ];
        let texture_cords:Vec<Vec2> = vec![
            vec2(1.0, 0.0),
            vec2(0.0, 0.0),
            
            vec2(1.0, 0.0),
            vec2(0.0, 0.0,),
            

            vec2(0.5, 1.0)
        ];

        
        Mesh { vertices, indices, color: None, texture_cords:Some(texture_cords) }
    }
    pub fn triangle(width:f32,height:f32,depth:f32) -> Mesh
    {
        let vertices  = vec![
            vec3(width / 2.0,  -height / 2.0,    depth),  
            vec3(-width / 2.0,  -height / 2.0,    depth),  
            vec3(0.0,   height / 2.0,    depth),   
           
        ];
        let texture_cords:Vec<Vec2> = vec![
            vec2(1.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.5, 1.0),
        ];
        let indices:Vec<u32>= vec!
        [
            0,1,2
        ];
        Mesh { vertices, indices, color: None, texture_cords:Some(texture_cords) }
    }
}
