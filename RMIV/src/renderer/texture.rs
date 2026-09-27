use std::ffi::c_void;

pub struct Texture
{
    pub id:u32,
}

impl Texture
{
    pub fn create(texture_path:&str) -> Texture{
        let img1 = image::open(texture_path);
        let img =  match img1 {
            Ok(file) => file,
            Err(error) => 
            {
                println!("failed to open file due to {}", error);
                image::open("Assets/textures/NoTexture.png").expect("hello") 
            },
        };
        let img = img.flipv().into_rgba8();
        let (iwidth,iheight) = img.dimensions();
        let data = img.as_raw();
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TextureParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TextureParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);

            gl::TextureParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TextureParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0,gl::RGBA as i32, iwidth as i32, iheight as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
        
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        
       
        
        Texture
        {
            id:texture,
        }
    
    }
}

/*impl Drop for Texture
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
*/
