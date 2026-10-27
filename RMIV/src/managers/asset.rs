use crate::backend::*;

pub trait Asset {
    fn add_to(self, assets: &mut AssetManager,handle:&AssetHandle);

    
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,)]
pub struct AssetHandle
{
    id:u32,
    generation:u32
}
pub struct Texture
{
    pub path:String,
}
#[derive(Clone)]
pub struct CustomShader
{
    pub vertex_path:String,
    pub fragment_path:String,
}

impl CustomShader
{
    pub fn defaultshader() -> CustomShader
    {
        CustomShader { vertex_path: "default".to_string(), fragment_path: "default".to_string() }
    }
}

pub struct Storage<T,U> {
    handles: Vec<AssetHandle>,
    assetdata: Vec<T>,
    asset: Vec<U>,
    asset_to_index: Vec<Option<usize>>,
}
pub struct AssetManager
{
    pub textures:Storage<Texture,TextureInstance>,
    pub shaders:Storage<CustomShader,ShaderInstance>,
    pub meshes:Storage<Mesh,MeshInstance>,
    freehandles:Vec<AssetHandle>,
    index:u32
}
impl Asset for Texture
{
    fn add_to(self, assets: &mut AssetManager,handle:&AssetHandle)
    {
        assets.textures.asset.push(TextureInstance::create(&self.path));
        assets.textures.handles.push(handle.clone());
        assets.textures.assetdata.push(self);
        assets.textures.asset_to_index[handle.id as usize] = Some(assets.textures.assetdata.len() -1);
    }
 
}
impl Asset for CustomShader
{
    fn add_to(self, assets: &mut AssetManager,handle:&AssetHandle)
    {
        if self.vertex_path == "default" && self.fragment_path == "default"
        {
            assets.shaders.asset.push(ShaderInstance::create_default());
            assets.shaders.handles.push(handle.clone());
            assets.shaders.assetdata.push(self);
            assets.shaders.asset_to_index[handle.id as usize] = Some(assets.shaders.assetdata.len() -1);
            return;
        }
        assets.shaders.asset.push(ShaderInstance::create(&self.vertex_path,& self.fragment_path));
        assets.shaders.handles.push(handle.clone());
        assets.shaders.assetdata.push(self);
        assets.shaders.asset_to_index[handle.id as usize] = Some(assets.shaders.assetdata.len() -1);
    }
}
impl Asset for Mesh
{
   fn add_to(self, assets: &mut AssetManager,handle:&AssetHandle)
    {
        assets.meshes.asset.push(MeshInstance::gendata(&self));
        assets.meshes.handles.push(handle.clone());
        assets.meshes.assetdata.push(self);
        assets.meshes.asset_to_index[handle.id as usize] = Some(assets.meshes.assetdata.len() -1);
    }
 
}
impl <A:Asset,B:Asset> Asset for (A,B) {
    fn add_to(self, assets: &mut AssetManager,handle:&AssetHandle)
    {
        self.0.add_to(assets, handle);
        self.1.add_to(assets, handle);
    }
}

impl <A:Asset,B:Asset,C:Asset> Asset for (A,B,C) {
    fn add_to(self, assets: &mut AssetManager,handle:&AssetHandle)
    {
        self.0.add_to(assets, handle);
        self.1.add_to(assets, handle);
        self.2.add_to(assets, handle);
    }

}


impl <T,U>Storage<T,U>
{
    pub fn new() -> Storage<T,U>
    {
        Storage { handles: Vec::new(), assetdata: Vec::new(),asset:Vec::new(), asset_to_index: Vec::new() }
    }
    pub fn get_asset(&self,entity:&AssetHandle) -> Option<&U>
    {
        if let Some(index) = self.asset_to_index[entity.id as usize]
        {
            if self.handles[index] == *entity
            {
                return Some(&self.asset[index]);
            }
            return None;
        }
        return None;
        
    }
    pub fn get_assetdata(&self,entity:&AssetHandle) -> Option<&T>
    {
        if let Some(index) = self.asset_to_index[entity.id as usize]
        {
            if self.handles[index] == *entity
            {
                return Some(&self.assetdata[index]);
            }
            return None;
        }
        return None;
        
    }
    pub fn destroy(&mut self,entity:&AssetHandle) -> bool
    {
        let index = self.asset_to_index[entity.id as usize];
        match index {
            Some(pos) => {
                self.asset_to_index[self.handles[self.handles.len() -1].id as usize] = Some(pos);
                self.handles.swap_remove(pos);
                self.assetdata.swap_remove(pos);
                self.asset.swap_remove(pos);
                self.asset_to_index[entity.id as usize] = None;
                return true;
            },
            None =>{false},
        }
    }
}
impl AssetManager
{
    pub fn new() -> AssetManager
    {
        AssetManager 
        { 
            textures: Storage::new(), 
            shaders: Storage::new(), 
            meshes:Storage::new(),
            freehandles:Vec::new(),
            index:0
        }
    }
    pub fn load<T:Asset>(&mut self,asset:T) -> AssetHandle
    {
        if self.freehandles.len() > 0
        {
            let mut handle = self.freehandles.remove(0);
            handle.generation += 1;
            asset.add_to(self,&handle);
            handle
        }
        else 
        {
            let entity = AssetHandle { id: self.index, generation: 0 };
            self.textures.asset_to_index.push(None);
            self.shaders.asset_to_index.push(None);
            self.meshes.asset_to_index.push(None);
            self.index += 1;
            asset.add_to(self,&entity);
            entity
        }
        
    }
    pub fn unload_all_textures(&mut self)
    {
        for i in &self.textures.handles.clone()
        {
            self.unload_texture(i);
        }
    }
    pub fn unload_all_shaders(&mut self)
    {
        for i in &self.shaders.handles.clone()
        {
            self.unload_shader(i);
        }
    }
     pub fn unload_all_meshes(&mut self)
    {
        for i in &self.meshes.handles.clone()
        {
            self.unload_mesh(i);
        }
    }
    
    pub fn unload_shader(&mut self,handle:&AssetHandle)
    {
        self.shaders.destroy(handle);
    }
    pub fn unload_mesh(&mut self,handle:&AssetHandle)
    {
        self.meshes.destroy(handle);
    }
    pub fn unload_texture(&mut self,handle:&AssetHandle)
    {
        self.textures.destroy(handle);
    }

    pub fn unload_all(&mut self)
    {
        self.unload_all_meshes();
        self.unload_all_shaders();
        self.unload_all_textures();
    }

    pub fn unload(&mut self,handle:AssetHandle)
    {
        if self.textures.destroy(&handle) || self.shaders.destroy(&handle) || self.meshes.destroy(&handle)
        {
            self.freehandles.push(handle);
        }
        
    }
    pub fn reload_shader(&mut self,handle:&AssetHandle)
    {
        let index = self.shaders.asset_to_index[handle.id as usize];
        match index {
            Some(pos) => {
                self.shaders.asset[pos] = ShaderInstance::create(&self.shaders.assetdata[pos].vertex_path,&self.shaders.assetdata[pos].fragment_path);
            },
            None =>{},
        }

    }
    pub fn reload_mesh(&mut self,handle:&AssetHandle)
    {
         let index = self.meshes.asset_to_index[handle.id as usize];
        match index {
            Some(pos) => {
                self.meshes.asset[pos] = MeshInstance::gendata(&self.meshes.assetdata[pos]);
            },
            None =>{},
        }
    }
    pub fn reload_texture(&mut self,handle:&AssetHandle)
    {
        let index = self.textures.asset_to_index[handle.id as usize];
        match index {
            Some(pos) => {
                self.textures.asset[pos] = TextureInstance::create(&self.textures.assetdata[pos].path);
            },
            None =>{},
        }
    }
    pub fn reload_all_textures(&mut self)
    {
        for i in &self.textures.handles.clone()
        {
            self.reload_texture(i);
        }
    }
    pub fn reload_all_shaders(&mut self)
    {
        for i in &self.shaders.handles.clone()
        {
            self.reload_shader(i);
        }
    }
    pub fn reload_all_meshes(&mut self)
    {
        for i in &self.meshes.handles.clone()
        {
            self.reload_mesh(i);
        }
    }

    pub fn reload_all_assets(&mut self)
    {
        self.reload_all_meshes();
        self.reload_all_shaders();
        self.reload_all_textures();
    }



}
impl Drop for AssetManager
{
    fn drop(&mut self) {
        self.unload_all();
    }
}


