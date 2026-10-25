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



pub struct Storage<T,U> {
    handles: Vec<AssetHandle>,
    assetdata: Vec<T>,
    asset: Vec<U>,
    asset_to_index: Vec<Option<usize>>,
}
pub struct AssetManager
{
    pub textures:Storage<Texture,TextureInstance>,
    pub shaders:Storage<CustomShader,Shader>,
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
        assets.shaders.asset.push(Shader::create(&self.vertex_path,& self.fragment_path));
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
    pub fn destroy(&mut self,entity:&AssetHandle)
    {
        let index = self.asset_to_index[entity.id as usize];
        match index {
            Some(pos) => {
                self.asset_to_index[self.handles[self.handles.len() -1].id as usize] = Some(pos);
                self.handles.swap_remove(pos);
                self.assetdata.swap_remove(pos);
                self.asset.swap_remove(pos);
                self.asset_to_index[entity.id as usize] = None;
                
            },
            None =>{},
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
    pub fn unload(&mut self,handle:AssetHandle)
    {
        self.textures.destroy(&handle);
    }



}



