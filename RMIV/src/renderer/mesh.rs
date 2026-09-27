use gl;
use std:: ffi::c_void;

pub struct Mesh{
    pub vbo:u32,
    pub vao:u32,
    pub ebo:u32,
}

impl Mesh{
    pub fn gendata(vertices:[f32;20], indices:[u32;6]) -> Mesh
    {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * vertices.len()) as isize,vertices.as_ptr() as *const c_void,gl::STATIC_DRAW);
        }

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,(std::mem::size_of::<f32>() * 5) as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE,(std::mem::size_of::<f32>() * 5) as i32,(std::mem::size_of::<f32>()*3) as *const c_void);
            gl::EnableVertexAttribArray(1);



        }

        let mut ebo = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (std::mem::size_of::<u32>() * indices.len()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

        }
        Mesh
        { 
            vbo, 
            vao, 
            ebo 
        }
    }
    pub fn mesh_bind(&self)
    {
        unsafe{gl::BindVertexArray(self.vao);}
    }
}
/*impl Drop for Mesh
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
*/


