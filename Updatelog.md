# RMIV Engine Update Log

**RMIV Engine** is a work-in-progress game engine written in Rust using OpenGL. 


**Updates**
```sh
There have been updates to the following systems. Asset Manager, Window Manager(including platform), World and 3D implementation.
The locations in which Open gl objects such as meshes and windows are stored are now called (type)Instance for example WindowInstance, MeshInstance, TextureInstance etc.
**3D** is now supported through the camera by calling the Camera3D creation function the asset manager has also been updated to allow for 3d meshes 
```


# AssetManager
> the asset manager now supports loading of meshes, textures and shaders returning an asset handle which can be used during entity creation
> data is now stored more sufficiently in the asset manager than before using a similar system to world
> now manages mesh creation 

# WindowManager
> Windows no longer own meshes however issues pertaining mesh sharing across windows now exist this will hopefully be patched in the next update. 
> windows are now also stored similarly to world

# Meshes
> mesh templates which include a cubeoid, rectangular based pyramid,triangle and a rectangle which all allow you to choose length and width
> automatic mesh creation through the asset manager only requiring a mesh preset stated earlier or by specify verities and indices in a mesh struct.
> meshes can optionally have color or textures with any amount of vertices being supported for them. Entities no longer require a texture


# World
> Camera objects now have 2 templates a camera 2D which uses an orthographic projection and a camera 3D which uses perspective projection allowing for 3D. Systems for manipulating the camera have yet to be implemented but will surely be added in the next update.
> Added collision checks for the z axis collision checks for custom meshes will be added in the next update
