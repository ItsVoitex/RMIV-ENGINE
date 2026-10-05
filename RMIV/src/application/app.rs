use gl;
use glam::u32;
use glfw::{fail_on_errors,Context};


use std::{collections::HashMap, ffi::c_void};
use crate::{managers::{asset::AssetManager, input::{Action, Key,MouseButton}}, templates::object::Object};
use crate::application::window::Window;
use crate::renderer::Renderer;
use std::mem;


pub struct App
{
    pub assets:AssetManager,
    pub renderer: HashMap<u32,Option<Renderer>>,
    pub windows:HashMap<u32,Option<Window>>,
    pub window:Window,
    pub glfw:glfw::Glfw,
    idcounter:u32,
    bound_window:u32,
    
}

impl App
{
    pub fn new() -> App
    {
        let mut glfw = glfw::init(fail_on_errors).unwrap();
        
        let (window,_events) = glfw.create_window(1, 1, "primary", glfw::WindowMode::Windowed).expect("failed");

        let mut window = Window::basic_window(window, _events);

        window.window.make_current();

        gl::load_with(|symbol| {
            window.window.get_proc_address(symbol)
            .map_or(std::ptr::null(), |f| f as *const c_void)
        });

        window.window.set_framebuffer_size_polling(true);

        window.window.hide();
        
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
        }

        App {
            glfw, 
            windows:HashMap::new(),
            renderer:HashMap::new(),
            assets:AssetManager::new(),
            window:window,
            idcounter:0,   
            bound_window:0
        }
        
    }
    pub fn create_window(&mut self,width:u32,height:u32,window_title:&str) -> &mut App
    {
        let mut window: Window;
        window = Window::create_shared(self,width,height,window_title);
        window.window.make_current();
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        
        window.window.set_framebuffer_size_polling(true);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
        }
        self.windows.insert(self.idcounter, Some(window));
        self.renderer.insert(self.idcounter, Some(Renderer::new()));
        self.idcounter += 1;
        self
    }

    //makes a window and creates an open gl context but reuqires a primary window id 
    
    //makes calls to the renderer
    pub fn queue_draw(&mut self,objects:Vec<&Object>) 
    {
        if let Some(render) = self.renderer.get_mut(&self.bound_window)
        {
            match render {
                Some(render) => 
                {
                    render.draw_objects(&mut self.assets, objects);
                }
                None => return
                
            }
        }
        else {
            panic!("failed to find window with id {}" , self.bound_window);
        }
    }
    pub fn draw(&mut self,objects:&mut Object) 
    {
        if let Some(render) = self.renderer.get_mut(&self.bound_window)
        {
             match render {
                Some(render) => 
                {
                    render.draw_object(objects,&mut self.assets, );
                }
                None => return    
            }
            
        }
        else {
            panic!("failed to find window with id {}" , self.bound_window);
        }
    }
    
    
    //calculates the frame time (duration between frames) to act as a constant for actions such as moving objects positions
    pub fn delta_time(&mut self,window_id:u32) -> f32
    {
        if let Some(window) = self.windows.get_mut(&window_id)
        {
            match window {
                Some(wind) =>return wind.delta_time(self.glfw.get_time()),
                None => return 0.0
            }
            
        }
        else {
            panic!("unable to find window with id: {}",window_id);
        }
    }

    //checks if window has been closed kind of self explanatory
    pub fn window_should_close(&mut self,id:u32) ->bool
    {
        if let Some(window) = self.windows.get_mut(&id)
        {
            match window {
                Some(wind) =>{
                    return wind.window.should_close();
                },
                None => return true
            }
        }
        else {
            panic!("no window with that id");
        }
        
    }


    //resets buffers and selets the specified window to being drawing to
    pub fn begin_drawing(&mut self,window_id:u32)
    {
        if let Some(window) = self.windows.get_mut(&window_id)
        {
            match window {
                Some(wind) =>{
                    wind.window.make_current();
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::Viewport(0, 0, wind.frame_buffer.0, wind.frame_buffer.1);
                }
                self.bound_window = window_id;
                },
                None => return
            }
        }
        else{
            panic!("unable to find window with id {}",window_id);
        }
        
    }
    //swaps the previous frame with the new frame
    pub fn end_drawing(&mut self)
    {
        if let Some(window) = self.windows.get_mut(&self.bound_window)
        {
            match window {
                Some(wind) =>{
                    wind.window.make_current();
                    wind.window.swap_buffers();
                    self.window.window.make_current();
                },
                None => {self.window.window.make_current();return}
            }
        }
        else{
            panic!("unable to find window with id: {}",self.bound_window); 
        }
        
    }

    pub fn update_events(&mut self)
    {
        let mut remove_queue: Vec<u32> = Vec::new();
        self.glfw.poll_events();
        for (index,window) in self.windows.iter_mut()
        {
            match window {
                Some(wind) =>{
                    if wind.process_events() || wind.window.should_close()
                    {
                        remove_queue.push(index.clone());
                    }
                },
                None => continue
            }
            
        }
        self.window.window.make_current();
               
        
        for i in remove_queue.iter()
        {
            
            self.renderer.insert(*i, None);
            self.windows.insert(*i,None);

        }

        
    }
    
     


    //use for input takes window id as paramter
    fn get_mouse(&self,window_id:u32, button: MouseButton) -> Option<Action>
    {
        if let Some(window) = self.windows.get(&window_id)
        {
            match window {
                Some(wind) => unsafe { mem::transmute(glfw::ffi::glfwGetMouseButton(wind.window.window_ptr(), button as std::os::raw::c_int)) },
                None=> None,
            }
        }
        else {
            panic!("unable to find window with id: {}",window_id); 
        }
        
    }
    pub fn mouse_is_pressed(&self,window_id:u32, button: MouseButton) -> bool
    {
        if (self.get_mouse(window_id,button)) == Some(Action::Press)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn mouse_is_released(&self,window_id:u32, button: MouseButton) -> bool
    {
        if (self.get_mouse(window_id,button)) == Some(Action::Release)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn mouse_is_repeated(&self,window_id:u32, button: MouseButton) -> bool
    {
        if (self.get_mouse(window_id,button)) == Some(Action::Repeat)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    fn get_key(&self,window_id:u32, key: Key) -> Option<Action> {
        if let Some(window) = self.windows.get(&window_id)
        {
            match window {
                Some(wind) => unsafe { std::mem::transmute(glfw::ffi::glfwGetKey(wind.window.window_ptr(), key as std::os::raw::c_int)) }
                None=> None,
            }
        }
        else {
            panic!("unable to find window with id: {}",window_id); 
        }
    }
    pub fn key_is_pressed(&self,window_id:u32, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Some(Action::Press)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn key_is_released(&self,window_id:u32, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Some(Action::Release)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    pub fn key_is_repeated(&self,window_id:u32, key: Key) -> bool
    {
        if (self.get_key(window_id,key)) == Some(Action::Repeat)
        {
            return true;
        }
        else {
            return  false;
        }
    }



    //makes calls to the asset manager
    pub fn load_textured_mesh(&mut self,texture_path:&str,width:f32,height:f32,depth:f32,window_id:u32) -> Option<(u32,u32)>
    {
        if let Some(window) = self.windows.get_mut(&window_id)
        {
            match window {
                Some(wind) =>{
                    wind.window.make_current();
                    return Some(self.assets.load_textured_mesh(texture_path, width, height, depth))
                    
                },
                None => None
            }
            
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

    pub fn load_mesh(&mut self,width:f32,height:f32,depth:f32,window_id:u32) -> Option<u32>
    {
        if let Some(window) = self.windows.get_mut(&window_id)
        {
            match window {
                Some(wind) =>{
                    wind.window.make_current();
                    return Some(self.assets.load_mesh(width, height, depth))
                    
                },
                None => None
            }
        }
        else {
            panic!("failed to find window with that id {}",window_id);
        }
        
    }

   
    
}
    
