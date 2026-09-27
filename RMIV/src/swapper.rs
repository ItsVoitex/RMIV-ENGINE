
use crate::renderer::texture;
use crate::renderer::texture::Texture;

pub struct TextureSwapper{
        pub textures:Vec<Texture>
    }

impl TextureSwapper
{
    pub fn create(path:&[&str])->TextureSwapper
    {
        let mut textures = Vec::new();
        for i in 0..path.len()
        {
            let texture = texture::Texture::create(path[i]);
            textures.push(texture);


        }
        TextureSwapper { 
            textures
        }
        
    }
   
}
