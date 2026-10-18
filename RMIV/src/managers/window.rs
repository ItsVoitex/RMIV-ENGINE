




use glfw::Glfw;
use crate::renderer::renderer::deafultmesh;
use glfw::{Context, GlfwReceiver,WindowEvent};

use crate::renderer::{Mesh};


#[derive(Clone, Copy)]
pub struct  WindowHandle
{
    pub id:u32,
    pub is_primary:bool
}


pub struct WindowManager
{
    pub sharedwindows:Vec<Option<Window>>,
    pub primarywindows:Vec<Option<Window>>,
    pub primarymeshes:Vec<Option<Mesh>>,
    pub sharedmeshes:Vec<Option<Mesh>>,
    pub mainwind:Window,
}
pub struct Window 
{
    pub window:glfw::PWindow,
    pub events:GlfwReceiver<(f64,WindowEvent)>,
    pub delta_time:f32,
    last_time:f64,
    current_time:f64,
    pub frame_buffer:(i32,i32),
    pub windinfo:(i32,i32,bool,bool),
    is_primary:bool,
}
impl WindowManager
{
    pub fn new(runtime:&mut Glfw) -> WindowManager
    {
        let mut mainwind = Window::create_primary(runtime, 1, 1, "mainwind");
        mainwind.window.hide();
        WindowManager { sharedwindows: Vec::new(), primarywindows: Vec::new(), mainwind: mainwind,primarymeshes:Vec::new(),sharedmeshes:Vec::new() }
    }
    
    pub fn create_window(&mut self,runtime:&mut Glfw,width:u32,height:u32,window_title:&str) -> WindowHandle
    {
        let wind = &self.mainwind;
        let window = Window::create_shared(width, height, window_title, runtime,&wind);
        self.sharedwindows.push(Some(window));


        self.sharedmeshes.push(Some(deafultmesh()));
        
        return WindowHandle { id: self.sharedwindows.len() as u32 -1, is_primary: false }; 
    }
    pub fn create_shared(&mut self,runtime:&mut Glfw,primarywindow:&WindowHandle,width:u32,height:u32,window_title:&str) -> WindowHandle
    {
        if let Some(wind) = &self.primarywindows[primarywindow.id as usize]
        {
            let window = Window::create_shared(width, height, window_title, runtime,&wind);
            self.sharedwindows.push(Some(window));

            self.sharedmeshes.push(Some(deafultmesh()));
        
            return WindowHandle { id: self.sharedwindows.len() as u32 -1, is_primary: false };
        }
        else {
            panic!("window handle for primary window is not valid{}",primarywindow.id)
        }  
    }
    pub fn create_primary(&mut self,runtime:&mut Glfw,width:u32,height:u32,window_title:&str) -> WindowHandle
    {
        let window = Window::create_primary(runtime,width, height, window_title);
        self.primarywindows.push(Some(window));

        self.primarymeshes.push(Some(deafultmesh()));

        return WindowHandle { id: self.primarywindows.len() as u32 -1, is_primary: true };
    }
    pub fn update_events(&mut self)
    {
        
        let mut remove_queue1: Vec<usize> = Vec::new();
        let mut remove_queue2: Vec<usize> = Vec::new();
        for i in 0..self.sharedwindows.len()
        {   
            if let Some(wind) = &mut self.sharedwindows[i]
            {
                wind.toggle_fullscreen();
                if wind.process_events() || wind.should_close()
                {
                    remove_queue1.push(i);
                }
            }
            else{
                continue;
            }
        }
        for i in 0..self.primarywindows.len()
        {   
            if let Some(wind) = &mut self.primarywindows[i]
            {
                wind.toggle_fullscreen();
                if wind.process_events() || wind.should_close()
                {
                    remove_queue2.push(i);
                }
            }
            else{
                continue;
            }
        }

        self.mainwind.make_current();
        
               
        
        if remove_queue1.len() > 0 || remove_queue2.len() > 0
        {
            for i in 0..remove_queue1.len()
            {
                self.sharedwindows[remove_queue1[i]] = None;
                self.sharedmeshes[remove_queue1[i]] = None;

            }
            for i in 0..remove_queue2.len()
            {
                self.primarywindows[remove_queue2[i]] = None;
                self.primarymeshes[remove_queue2[i]] = None;

            }
        }
        
    }
    pub fn get(&self,handle:&WindowHandle) -> Option<&Window>
    {
        if handle.is_primary == true
        {
            if (handle.id as usize) < self.primarywindows.len()
            {
                let val = &self.primarywindows[handle.id as usize];
                match val{
                    Some(val) => return Some(val),
                    None=> return None,
                }
            }
            else {
                panic!("failed to find window with that id out of bounds error {}",handle.id)
            }
        }
        else {
            if (handle.id as usize) < self.sharedwindows.len()
            {
                let val = &self.sharedwindows[handle.id as usize];
                match val{
                    Some(val) => return Some(val),
                    None=> return None,
                }
            }
            else {
                panic!("failed to find window with that id out of bounds error {}",handle.id)
            }
        }
    }
    pub fn get_mesh(&mut self,handle:&WindowHandle) -> Option<&mut Mesh>
    {
        if handle.is_primary == true
        {
            if (handle.id as usize) < self.primarymeshes.len()
            {
                let val = &mut self.primarymeshes[handle.id as usize];
                match val{
                    Some(val) => return Some(val),
                    None=> return None,
                }
            }
            else {
                panic!("failed to find mesh with that id out of bounds error {}",handle.id)
            }
        }
        else {
            if (handle.id as usize) < self.sharedmeshes.len()
            {
                let val = &mut self.sharedmeshes[handle.id as usize];
                match val{
                    Some(val) => return Some(val),
                    None=> return None,
                }
            }
            else {
                panic!("failed to find mesh with that id out of bounds error {}",handle.id)
            }
        }
    
    }
    pub fn get_mut(&mut self,handle:&WindowHandle) -> Option<&mut Window>
    {
        if handle.is_primary == true
        {
            if (handle.id as usize) < self.primarywindows.len()
            {
                let val = &mut self.primarywindows[handle.id as usize];
                match val{
                    Some(val) => return Some(val),
                    None=> return None,
                }
            }
            else {
                panic!("failed to find window with that id out of bounds error {}",handle.id)
            }
        }
        else {
            if (handle.id as usize) < self.sharedwindows.len()
            {
                let val = &mut self.sharedwindows[handle.id as usize];
                match val{
                    Some(val) => return Some(val),
                    None=> return None,
                }
            }
            else {
                panic!("failed to find window with that id out of bounds error {}",handle.id)
            }
        }
    }
}

impl Window
{
    pub fn create_window(width:u32,height:u32,window_title:&str,runtime:&mut Glfw,primarywindow:&Window) -> Window
    {
        if let Some((mut window,_events)) =  primarywindow.window.create_shared(width, height, window_title, glfw::WindowMode::Windowed)
            {
                let mut width:std::ffi::c_int = 0;
                let mut height:std::ffi::c_int = 0;
                window.make_current();
                runtime.set_swap_interval(glfw::SwapInterval::Sync(1));
                window.set_framebuffer_size_polling(true);
                unsafe {
                    glfw::ffi::glfwGetFramebufferSize(window.window_ptr(), &mut width, &mut height);
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::Enable(gl::DEPTH_TEST);
                    
                }
                return Window { window, events: (_events), delta_time:0.0,last_time:0.0,current_time:0.0,frame_buffer:(width,height),is_primary:false,windinfo:(width as i32,height as i32,false,false)};    
            }
            else {
                panic!("failed to create shared window");
            }
    }
    pub fn create_primary(runtime:&mut Glfw,width:u32,height:u32,window_title:&str) -> Window
    {
       
       
        let (mut window,_events) = runtime.create_window(width, height, window_title, glfw::WindowMode::Windowed).expect("failed to create primary window");
        
        window.make_current();
        gl::load_with(|symbol| {
            window.get_proc_address(symbol)
            .map_or(std::ptr::null(), |f| f as *const std::ffi::c_void)
        });
        runtime.set_swap_interval(glfw::SwapInterval::Sync(1));
        window.set_framebuffer_size_polling(true);
        let mut width:std::ffi::c_int = 0;
        let mut height:std::ffi::c_int = 0;
        unsafe {
            glfw::ffi::glfwGetFramebufferSize(window.window_ptr(), &mut width, &mut height);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
        }

        Window { 
            window, 
            events: (_events), 
            delta_time:0.0,
            last_time:0.0,
            current_time:0.0,
            frame_buffer:(width,height),
            windinfo:(width as i32,height as i32,false,false),
            is_primary:true
        }
    }
    
    //calculates frame time for the specific window
    pub fn delta_time(&mut self,current_time:f64) ->f32
    {
        self.current_time = current_time;
        self.delta_time = (self.current_time - self.last_time) as f32;
        self.last_time = self.current_time;
        return self.delta_time;
    }
    pub fn create_shared(width:u32,height:u32,window_title:&str,runtime:&mut Glfw,primarywindow:&Window) -> Window
    {
        
         if let Some((mut window,_events)) =  primarywindow.window.create_shared(width, height, window_title, glfw::WindowMode::Windowed)
            {
                let mut width:std::ffi::c_int = 0;
                let mut height:std::ffi::c_int = 0;
                window.make_current();
                runtime.set_swap_interval(glfw::SwapInterval::Sync(1));
                window.set_framebuffer_size_polling(true);
                unsafe {
                    glfw::ffi::glfwGetFramebufferSize(window.window_ptr(), &mut width, &mut height);
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::Enable(gl::DEPTH_TEST);
                    
                }
                return Window { window, events: (_events), delta_time:0.0,last_time:0.0,current_time:0.0,frame_buffer:(width,height),is_primary:false,windinfo:(width as i32,height as i32,false,false)};    
            }
            else {
                panic!("failed to create shared window");
            }
        
    }
    pub fn process_events(&mut self) -> bool
    {
        let mut should_remove = false;
    
        for (_,events) in glfw::flush_messages(&self.events)
        {
            match events {
                glfw::WindowEvent::Close => {self.window.set_should_close(true);should_remove = true},

                glfw::WindowEvent::FramebufferSize(width,height ) => 
                {
                    if !self.window.should_close() && !should_remove
                    {
                        self.frame_buffer.0 = width;
                        self.frame_buffer.1 = height;
                    }
                },
                _ => {}
            }
        }
        return should_remove;
    }
    pub fn should_close(&mut self) ->bool
    {
        return self.window.should_close();
    }
    pub fn close(&mut self)
    {
        self.window.set_should_close(true);
    }
    pub fn toggle_fullscreen(&mut self)
    {
        unsafe {
            let monitor = glfw::ffi::glfwGetPrimaryMonitor();
            let mode= glfw::ffi::glfwGetVideoMode(monitor);
           
            
            let pressed = glfw::ffi::GLFW_PRESS == glfw::ffi::glfwGetKey(self.window.window_ptr(),glfw::ffi::GLFW_KEY_F11 as std::os::raw::c_int);
            if pressed && self.windinfo.2 == true && !self.windinfo.3
            {
                glfw::ffi::glfwSetWindowMonitor(self.window.window_ptr(),std::ptr::null_mut(), (*mode).width-self.windinfo.0 / 2, (*mode).height-self.windinfo.1 / 2, self.windinfo.0, self.windinfo.1, 0);
                self.windinfo.2 = false;
  
            }
            else {if pressed && self.windinfo.2 == false && !self.windinfo.3 {
                glfw::ffi::glfwSetWindowMonitor(self.window.window_ptr(), monitor, 0, 0, (*mode).width, (*mode).height, (*mode).refreshRate);
                self.windinfo.2 = true;
     
            }}
            
            self.windinfo.3 = pressed;
        }
        
    }
    pub fn make_current(&mut self)
    {
        self.window.make_current()
    }
    pub fn begin_drawing(&mut self)
    {
        self.make_current();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, self.frame_buffer.0, self.frame_buffer.1);
        }
        
        
        
    }
    pub fn end_drawing(&mut self)
    {
        self.window.swap_buffers();  
    }
}
impl Drop for Window {
    fn drop(&mut self) {
        
        unsafe {
            if !self.is_primary
            {
                glfw::ffi::glfwDestroyWindow(self.window.window_ptr());
            }
            
        }
    }
}
