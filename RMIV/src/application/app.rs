use gl;
use glfw::Context;

use std::{collections::HashMap, ffi::{CString, c_void}};
use crate::application::input::{Key,Action};
use crate::application::window::{self, Window};
use glfw::{fail_on_errors};

/*pub struct App {
    window: Window,
    time: Time,
    input: Input,
    renderer: Renderer,
    assets: AssetManager,
}
    assest manager creates a hashmap of textures so they can be reused
    window will conatin the events window and glfw
    input will be the definitions for glfw mouse inputs
    rednerer will be to abstract all open gl calls
    */
pub struct App
{
    
    pub glfw:glfw::Glfw,
    pub window:HashMap<String,Window>,
}

impl App
{
    pub fn new() -> App
    {
        let mut glfw = glfw::init(fail_on_errors).unwrap();
        
        
        
        


        App {
            glfw, 
            window:HashMap::new(), 
      
        }
        
    }
    pub fn create_window(&mut self,width:u32,height:u32,window_title:&str,window_id:&str)
    {
        let mut window = Window::create(&mut self.glfw,width,height,window_title);
        window.window.make_current();
        gl::load_with(|symbol| {
            window.window.get_proc_address(symbol)
            .map_or(std::ptr::null(), |f| f as *const c_void)
        });
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
        }
        let last_time:f64 =  self.glfw.get_time();
        self.window.insert(window_id.to_string(), window);
        

    }
    pub fn delta_time(&mut self,id:&str) -> f32
    {
        if let Some(window) = self.window.get_mut(id)
        {
            return window.delta_time(self.glfw.get_time());
        }
        else {
            panic!("unable to find window with id: {}",id);
        }
    }
    pub fn window_should_close(&mut self,id:&str) ->bool
    {
        //for when you have multiple windows
        if let Some(window) = self.window.get_mut(id)
        {
            return window.window.should_close();
        }
        else {
            panic!("unable to find window with id: {}",id);
        }
        
    }

    pub fn begin_drawing(&mut self,id:&str)
    {
        if let Some(window) = self.window.get_mut(id)
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
    pub fn end_drawing(&mut self,id:&str)
    {
        if let Some(window) = self.window.get_mut(id)
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
     fn get_key(&self,window_id:&str, key: Key) -> Action {
        if let Some(window) = self.window.get(window_id)
        {
            unsafe { std::mem::transmute(glfw::ffi::glfwGetKey(window.window.window_ptr(), key as std::os::raw::c_int)) }
        }
        else {
            panic!("unable to find window with id: {}",window_id); 
        }
    }


    pub fn key_is_pressed(&self,window_id:&str, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Action::Press
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn key_is_released(&self,window_id:&str, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Action::Release
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn key_is_repeated(&self,window_id:&str, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Action::Repeat
        {
            return true;
        }
        else {
            return  false;
        }
    }

   
    
}
    
