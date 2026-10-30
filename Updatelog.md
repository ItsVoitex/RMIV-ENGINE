<div align="center">

# RMIV Engine Update Log
</div>




## **RMIV Engine** work-in-progress game engine written in Rust using OpenGL. 









<div align="center">

## **Update 2**

</div>

```sh
There have been updates to the following systems. 
AssetManager,WindowManager,Platform,Mesh,World,Renderer and shaders.
there are more precise collision detection ,fullscreen issues are fixed, 
lots of optimisations, all axis rotation and many more.
```

### Notes
> rendering and game ticking isolation will be implemented when rendering can be separated to another thread.

> issues with timing on windows which causes inconsistent fps and tickrate due to thread sleep issues which will hopefully be fixed when a solituion for cpu cycle limiting is implemented.

### AssetManager
> more precise error messages when errors occur during asset loading/creation

### Platform/WindowManager
> fixed issues related to window relocation after full-screening the program by retraining the original window positions before fullscreen for platforms other than wayland which does not support setting window positions.

> implemented mouse cursor position translation to normalized coordinates to allow for future camera manipulation with the mouse and accurate clicking

> more precise error messages when errors occur during window creation and input handling

### World
> added the Color component which is a multiplier for color for the whole entity it uses values from 0-255 for red green blue values

> fixed bug in which after entity creation and calling the add component function the old entity data was overwritten due to no checks on existing entities during creation

> optimizations to reduce memory consumption by editing the renders snapshot of entities which have the required components for rendering instead of creating a new vector or pointers every time

> also more precise error messages when errors occur during collision checks and components requests

> finally precise hitbox detentions have been implemented to account for all axis rotation of an object by calculating their vertices after rotation and using the largest and smallest xyz values for bound checking



### Mesh
> implemented the following mesh templates cuboid, texturelesscube (meshes which use a solid color and no textures),square base pyramid and triangular based pyramid.


### Renderer
> now makes use of snapshots of entities from world to render interdependently from world.this is to optimise for memory and allow for multi-threading in the future

> render implements the debug option during creation which allows for the rendering of hitboxes to allow for visualisation of their size.

### Shader
> All axis rotation is now supported through the the transform component in its rotation property.
> the color multiplier has also been added.
> finally back face culling of faces that are not visible to the camera are added for example the inner faces of a cube.











<div align="center">

## **Update 1**
</div>

```sh
There have been updates to the following systems. Asset Manager, Window Manager(including platform), 
World and 3D implementation. The locations in which Open gl objects such as meshes and windows are stored 
are now called (type)Instance for example WindowInstance, MeshInstance, TextureInstance etc.
**3D** is now supported through the camera by calling the Camera3D creation function the 
asset manager has also been updated to allow for 3d meshes 
```


### AssetManager
> the asset manager now supports loading of meshes, textures and shaders returning an asset handle which can be used during entity creation
> data is now stored more sufficiently in the asset manager than before using a similar system to world
> now manages mesh creation 

### WindowManager
> Windows no longer own meshes however issues pertaining mesh sharing across windows now exist this will hopefully be patched in the next update. 
> windows are now also stored similarly to world

### Meshes
> mesh templates which include a cubeoid, rectangular based pyramid,triangle and a rectangle which all allow you to choose length and width
> automatic mesh creation through the asset manager only requiring a mesh preset stated earlier or by specify verities and indices in a mesh struct.
> meshes can optionally have color or textures with any amount of vertices being supported for them. Entities no longer require a texture


### World
> Camera objects now have 2 templates a camera 2D which uses an orthographic projection and a camera 3D which uses perspective projection allowing for 3D. Systems for manipulating the camera have yet to be implemented but will surely be added in the next update.
> Added collision checks for the z axis collision checks for custom meshes will be added in the next update
