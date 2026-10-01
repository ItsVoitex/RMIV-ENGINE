use gl;
use glam::u32;
use glfw::Context;

use std::{collections::HashMap, ffi::c_void};
use crate::{Object, managers::{asset::AssetManager, input::{Action, Key,MouseButton}}};
use crate::application::window::Window;
use crate::renderer::Renderer;
use glfw::{fail_on_errors};
use std::mem;

pub struct App
{
    pub assets:AssetManager,
    pub renderer: HashMap<u32,Renderer>,
    pub window:HashMap<u32,Window>,
    pub glfw:glfw::Glfw,
    idcounter:u32,
    functions:HashMap<String,fn(&mut App)>,
    
}

impl App
{
    pub fn new() -> App
    {
        let glfw = glfw::init(fail_on_errors).unwrap();
        
        App {
            glfw, 
            window:HashMap::new(),
            renderer:HashMap::new(),
            assets:AssetManager::new(),
            idcounter:0,
            functions:HashMap::new()
      
        }
        
    }
    pub fn create_window(&mut self,width:u32,height:u32,window_title:&str) -> &mut App
    {
        let mut window = Window::create(self,width,height,window_title);
        window.window.make_current();
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
        gl::load_with(|symbol| {
            window.window.get_proc_address(symbol)
            .map_or(std::ptr::null(), |f| f as *const c_void)
        });
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
        }
        
        self.window.insert(self.idcounter, window);
        self.renderer.insert(self.idcounter, Renderer::new());
        self.idcounter += 1;
        self
    }
    pub fn queue_draw(&mut self,objects:Vec<&Object>,window_id:u32) 
    {
        if let Some(render) = self.renderer.get_mut(&window_id)
        {
            render.draw_objects(&mut self.assets, objects);
        }
        else {
            panic!("failed to find window with id {}" , window_id);
        }
    }
    pub fn draw(&mut self,objects:&mut Object,window_id:u32) 
    {
        if let Some(render) = self.renderer.get_mut(&window_id)
        {
            render.draw_object(objects,&mut self.assets, );
        }
        else {
            panic!("failed to find window with id {}" , window_id);
        }
    }
    pub fn on_update(&mut self,function:fn(&mut App)) -> &mut App
    {
        self.functions.insert("update".to_string(), function);
        return self;
    }
    pub fn on_startup(&mut self,function:fn(&mut App)) -> &mut App
    {
        self.functions.insert("startup".to_string(), function);
        return self;
    }
    pub fn run(&mut self)
    {
        if let Some(func) = self.functions.get_mut("startup")
        {
                func(self);
        }

        while !self.window_should_close(0)
        {
            self.update_events();
            if let Some(func2) = self.functions.get_mut("update")
            {
                    func2(self);
            }
        }
    }
    pub fn delta_time(&mut self,id:u32) -> f32
    {
        if let Some(window) = self.window.get_mut(&id)
        {
            return window.delta_time(self.glfw.get_time());
        }
        else {
            panic!("unable to find window with id: {}",id);
        }
    }
    pub fn window_should_close(&mut self,id:u32) ->bool
    {
        if let Some(window) = self.window.get_mut(&id)
        {
            return window.window.should_close();
        }
        else {
            panic!("unable to find window with id: {}",id);
        }
        
    }

    pub fn begin_drawing(&mut self,id:u32)
    {
        if let Some(window) = self.window.get_mut(&id)
        {
            window.window.make_current();
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
        }
        else{
            println!("unable to find window with id {}",id);
        }
        
    }
    pub fn end_drawing(&mut self,id:u32)
    {
        if let Some(window) = self.window.get_mut(&id)
        {
            window.window.swap_buffers();
        }
        else{
            println!("unable to find window with id: {}",id); 
        }
        
    }
    pub fn update_events(&mut self)
    {
        self.glfw.poll_events();
    }
     
    fn get_mouse(&self,window_id:u32, button: MouseButton) -> Action
    {
        if let Some(window) = self.window.get(&window_id)
        {
            unsafe { mem::transmute(glfw::ffi::glfwGetMouseButton(window.window.window_ptr(), button as std::os::raw::c_int)) }
        }
        else {
            panic!("unable to find window with id: {}",window_id); 
        }
        
    }
    pub fn mouse_is_pressed(&self,window_id:u32, button: MouseButton) -> bool
    {
        if (self.get_mouse(window_id,button)) == Action::Press
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn mouse_is_released(&self,window_id:u32, button: MouseButton) -> bool
    {
        if (self.get_mouse(window_id,button)) == Action::Release
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn mouse_is_repeated(&self,window_id:u32, button: MouseButton) -> bool
    {
        if (self.get_mouse(window_id,button)) == Action::Repeat
        {
            return true;
        }
        else {
            return  false;
        }
    }
    fn get_key(&self,window_id:u32, key: Key) -> Action {
        if let Some(window) = self.window.get(&window_id)
        {
            unsafe { std::mem::transmute(glfw::ffi::glfwGetKey(window.window.window_ptr(), key as std::os::raw::c_int)) }
        }
        else {
            panic!("unable to find window with id: {}",window_id); 
        }
    }
    pub fn key_is_pressed(&self,window_id:u32, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Action::Press
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn key_is_released(&self,window_id:u32, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Action::Release
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn key_is_repeated(&self,window_id:u32, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Action::Repeat
        {
            return true;
        }
        else {
            return  false;
        }
    }

    pub fn load_textured_mesh(&mut self,texture_path:&str,width:f32,height:f32,depth:f32,window_id:u32) -> (u32,u32)
    {
        if let Some(window) = self.window.get_mut(&window_id)
        {
            window.window.make_current();
            self.assets.load_textured_mesh(texture_path, width, height, depth)
        }
        else {
            panic!("failed to find window with that id {}",window_id);
        }
        
    }

   
    
    pub fn load_texture_from_file(&mut self,texture_path:&str) -> u32
    {
        self.assets.load_texture_from_file(texture_path, )
    }

    pub fn load_shader_from_file(&mut self,shader_path:&[&str;2]) -> u32
    {
        self.assets.load_shader_from_file(shader_path, )
    }


    pub fn load_default_shader(&mut self) -> u32
    {
        self.assets.load_default_shader()
    }

    pub fn load_mesh(&mut self,width:f32,height:f32,depth:f32,window_id:u32) -> u32
    {
        if let Some(window) = self.window.get_mut(&window_id)
        {
            window.window.make_current();
            self.assets.load_mesh(width, height, depth, )
        }
        else {
            panic!("failed to find window with that id {}",window_id);
        }
        
    }

   
    
}
    
