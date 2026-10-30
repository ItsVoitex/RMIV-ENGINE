




use glfw::Glfw;
use glfw::{Context, GlfwReceiver,WindowEvent};



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,)]
pub struct  WindowHandle
{
    pub id:u32,
    pub generation:u32,
    pub is_primary:bool
}

pub struct Window {
    pub title:String,
    pub dimensions:(u32,u32),
    pub targetfps:i32,
    pub is_primary:bool,
    pub windowref:Option<WindowHandle>
}
pub struct WindowStorage {
    handles: Vec<WindowHandle>,
    pub windows: Vec<WindowInstance>,
    window_to_index: Vec<Option<usize>>,
}
pub struct WindowManager
{
    pub storage:WindowStorage,
    pub mainwind:WindowInstance,
    freehandles:Vec<WindowHandle>,
    nextid:u32,
}
pub struct WindowInstance 
{
    pub window:glfw::PWindow,
    events:GlfwReceiver<(f64,WindowEvent)>,
    delta_time:f32,
    last_time:f64,
    pub windinfo:WindowInfo,
    pub frameinfo:FrameInfo
    
}

pub struct WindowInfo
{
    olddimensions:(i32,i32),
    pub frame_buffer:(i32,i32),
    oldpositions:(i32,i32),
    is_fullscreen:bool,
    is_pressed:bool,
    pub is_primary:bool,

}
pub struct FrameInfo
{
    pub targetfps:i32,
    pub shouldrender:bool,
    last_frame:f64,
    last_second:f64,
    pub fps:i32,
}
impl WindowStorage
{
    pub fn new() -> WindowStorage
    {
        WindowStorage { handles: Vec::new(), windows: Vec::new(), window_to_index: Vec::new() }
    }
    pub fn get_window(&self,handle:&WindowHandle,fail_on_errors:bool) -> Result<&WindowInstance,&str>
    {
        if let Some(index) = self.window_to_index[handle.id as usize]
        {
            if self.handles[index] == *handle
            {
                return Ok(&self.windows[index]);
            }
            if fail_on_errors
            {
                panic!("[Program Error] failed to get window due to potentially expired entity handle")
            }
            return Err("[Program Error] failed to get window due to potentially expired entity handle");
        }
        if fail_on_errors
        {
            panic!("[Program Error] failed to get window either due to non existence or it has been closed")
        }
        return Err("[Program Error] failed to get window either due to non existence or it has been closed");
    }
    pub fn get_mut_window(&mut self,handle:&WindowHandle,fail_on_errors:bool) -> Result<&mut WindowInstance,&str>
    {
        if let Some(index) = self.window_to_index[handle.id as usize]
        {
            if self.handles[index] == *handle
            {
                return Ok(&mut self.windows[index]);
            }
            if fail_on_errors
            {
                panic!("[Program Error] failed to get window due to potentially expired entity handle")
            }
            return Err("[Program Error] failed to get window due to potentially expired entity handle");
        }
        if fail_on_errors
        {
            panic!("[Program Error] failed to get window either due to non existence or it has been closed")
        }
        return Err("[Program Error] failed to get window either due to non existence or it has been closed");
    }
           
    pub fn should_close(&mut self) ->bool
    {
        if self.windows.len() > 0
        {
            false
        }
        else {
            true
        }
    }
    pub fn destroy(&mut self,handle:&WindowHandle)
    {
        let index = self.window_to_index[handle.id as usize];
        match index {
            Some(pos) => {
                self.window_to_index[self.handles[self.handles.len() -1].id as usize] = Some(pos);
                let mut window = self.windows.swap_remove(pos);
                self.handles.swap_remove(pos);
                self.window_to_index[handle.id as usize] = None;
                window.close();
                
            },
            None =>{},
        }
    }
}
impl WindowManager
{
    pub fn new(runtime:&mut Glfw) -> WindowManager
    {
        let mut mainwind = WindowInstance::create_primary(runtime, 1, 1, "mainwind",0);
        mainwind.window.hide();
        WindowManager { mainwind, storage: WindowStorage::new(),nextid:0,freehandles:Vec::new()}
    }
    pub fn get(&self,windowhandle:&WindowHandle,fail_on_errors:bool) -> Result<&WindowInstance,&str>
    {
        self.storage.get_window(windowhandle,fail_on_errors)
    }
    pub fn get_mut(&mut self,windowhandle:&WindowHandle,fail_on_errors:bool) -> Result<&mut WindowInstance,&str>
    {
        self.storage.get_mut_window(windowhandle, fail_on_errors)
    }

    pub fn create_window(&mut self,runtime:&mut Glfw,windowdata:Window) -> WindowHandle
    {
        let mut _mainwindow:WindowInstance;
        if !windowdata.is_primary && windowdata.windowref.is_none()
        {
            let wind = &self.mainwind;
            _mainwindow = WindowInstance::create_shared(windowdata.dimensions.0, windowdata.dimensions.1, &windowdata.title, runtime,&wind,windowdata.targetfps);
        }
        else {
            if !windowdata.is_primary && let Some(windowref) = windowdata.windowref
            {
                let wind = self.storage.get_mut_window(&windowref,true);
                match wind {
                    Ok(wind) => _mainwindow = WindowInstance::create_shared(windowdata.dimensions.0, windowdata.dimensions.1, &windowdata.title, runtime,wind,windowdata.targetfps),
                    Err(error) => panic!("{}", error)
                }
                
            }
            else{
                if windowdata.is_primary 
                {
                    _mainwindow = WindowInstance::create_primary(runtime,windowdata.dimensions.0, windowdata.dimensions.1, &windowdata.title,windowdata.targetfps);
            
                }
                else
                {
                    panic!("[Program Error] failed to create window due missing or invalid windowdata for creation")
                } 
            }
        }
        if self.freehandles.len() > 0 
        {
            self.storage.windows.push(_mainwindow);
            let mut handle = self.freehandles.remove(0);
            handle.generation +=1;
            handle.is_primary = windowdata.is_primary;
            return handle;
        }
        else {
            let handle = WindowHandle{generation:0,id:self.nextid,is_primary:windowdata.is_primary};
            self.storage.windows.push(_mainwindow);
            self.storage.handles.push(handle.clone());
            self.storage.window_to_index.push(Some(self.storage.windows.len()-1));
            self.nextid+=1;
            return handle;

        }

    }
    pub fn update_events(&mut self,runtime:&mut glfw::Glfw,is_wayland:&bool)
    {
        
        let mut remove_queue: Vec<WindowHandle> = Vec::new();
        for i in 0..self.storage.windows.len()
        {   
            let wind = &mut self.storage.windows[i];
            wind.limitfps(runtime);
            wind.toggle_fullscreen(is_wayland);
    
            
            if wind.process_events() || wind.should_close()
            {
                remove_queue.push(self.storage.handles[i].clone());
            }
            
           
        }
        self.mainwind.make_current();
        
        if remove_queue.len() > 0
        {
            for i in 0..remove_queue.len()
            {
                self.storage.destroy(&remove_queue[i]);
                self.freehandles.push(remove_queue[i]);

            }
        }
       
        
    }
    
   
}

impl WindowInstance
{
    pub fn create_window(width:u32,height:u32,window_title:&str,runtime:&mut Glfw,primarywindow:&WindowInstance,targetfps:i32) -> WindowInstance
    {
        if let Some((mut window,_events)) =  primarywindow.window.create_shared(width, height, window_title, glfw::WindowMode::Windowed)
            {
                let mut width:std::ffi::c_int = 0;
                let mut height:std::ffi::c_int = 0;
                window.make_current();
                runtime.set_swap_interval(glfw::SwapInterval::Sync(0));
                window.set_framebuffer_size_polling(true);
                unsafe {
                    glfw::ffi::glfwGetFramebufferSize(window.window_ptr(), &mut width, &mut height);
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::Enable(gl::DEPTH_TEST);

                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT_FACE);
                    gl::FrontFace(gl::CCW);
                    
                }
                return WindowInstance { 
                    window,
                    events: (_events), 
                    delta_time:0.0,
                    last_time:0.0,
                    windinfo:WindowInfo{frame_buffer:(width,height),olddimensions:(width,height),oldpositions:(0,0),is_fullscreen:false,is_pressed:false, is_primary:false},
                    frameinfo:FrameInfo { targetfps, shouldrender: true, last_frame: 0.0, last_second: 0.0,fps: 0 }};    
            }
            else {
                panic!("[Fatal Error] failed to create shared window due to internal window creation libary error possibly an issue with the window instance");
            }
    }
    pub fn create_primary(runtime:&mut Glfw,width:u32,height:u32,window_title:&str,targetfps:i32) -> WindowInstance
    {
       
       
        let (mut window,_events) = runtime.create_window(width, height, window_title, glfw::WindowMode::Windowed).expect("failed to create primary window");
        
        window.make_current();
        gl::load_with(|symbol| {
            window.get_proc_address(symbol)
            .map_or(std::ptr::null(), |f| f as *const std::ffi::c_void)
        });
        runtime.set_swap_interval(glfw::SwapInterval::Sync(0));
        window.set_framebuffer_size_polling(true);
        let mut width:std::ffi::c_int = 0;
        let mut height:std::ffi::c_int = 0;
        unsafe {
            glfw::ffi::glfwGetFramebufferSize(window.window_ptr(), &mut width, &mut height);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::FRONT_FACE);
            gl::FrontFace(gl::CCW);
        }

        WindowInstance  { 
            window, 
            events: (_events), 
            delta_time:0.0,
            last_time:0.0,
            windinfo:WindowInfo{frame_buffer:(width,height) ,olddimensions:(width,height),oldpositions:(0,0),is_fullscreen:false,is_pressed:false,is_primary:true},
            frameinfo:FrameInfo { targetfps, shouldrender: true, last_frame: 0.0, last_second: 0.0, fps: 0 },
    
        }
    }
    
 
    pub fn delta_time(&mut self,current_time:f64) ->f32
    {
        self.delta_time = (current_time - self.last_time) as f32;
        self.last_time = current_time;
        return self.delta_time;
    }
    pub fn create_shared(width:u32,height:u32,window_title:&str,runtime:&mut Glfw,primarywindow:&WindowInstance,targetfps:i32) -> WindowInstance
    {
        
         if let Some((mut window,_events)) =  primarywindow.window.create_shared(width, height, window_title, glfw::WindowMode::Windowed)
            {
                let mut width:std::ffi::c_int = 0;
                let mut height:std::ffi::c_int = 0;
                window.make_current();
                runtime.set_swap_interval(glfw::SwapInterval::Sync(0));
                window.set_framebuffer_size_polling(true);
                unsafe {
                    glfw::ffi::glfwGetFramebufferSize(window.window_ptr(), &mut width, &mut height);
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::Enable(gl::DEPTH_TEST);

                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT_FACE);
                    gl::FrontFace(gl::CCW);
                    
                }
                return WindowInstance
                { 
                    window, 
                    events: (_events), 
                    delta_time:0.0,
                    last_time:0.0,
                    
                    windinfo:WindowInfo
                    {
                        olddimensions:(width,height),
                        frame_buffer:(width,height),
                        oldpositions:(0,0),
                        is_fullscreen:false,
                        is_pressed:false,
                        is_primary:false
                    },
                    frameinfo:FrameInfo 
                    {
                        targetfps,
                        shouldrender: true,
                        last_frame: 0.0,
                        last_second: 0.0,
                        fps: 0 
                    }
                };    
            }
            else {
                panic!("[Fatal Error] failed to create shared window due to internal window creation libary error possibly an issue with the window instance");
            }
        
    }
    
    pub fn limitfps(&mut self,runtime:&mut glfw::Glfw) -> f64
    {

       
        let frametime = 1.0/self.frameinfo.targetfps as f64; 
        let current_time = runtime.get_time();
        let mut wait_time = 0.0;
        if current_time - self.frameinfo.last_frame > frametime
        {
            self.frameinfo.shouldrender = true;
            self.frameinfo.last_frame = current_time;
        }
        else {
            self.frameinfo.shouldrender = false;
            wait_time = frametime - (current_time - self.frameinfo.last_frame) ;
        }
        
        if current_time - self.frameinfo.last_second > 1.0
        {
            println!("{} fps",self.frameinfo.fps);
            self.frameinfo.fps = 0;
            self.frameinfo.last_second = current_time;       
        }
        return wait_time;


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
                        self.windinfo.frame_buffer.0 = width;
                        self.windinfo.frame_buffer.1 = height;
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
    pub fn toggle_fullscreen(&mut self,is_wayland:&bool)
    {
        unsafe {
            
            let pressed = glfw::ffi::GLFW_PRESS == glfw::ffi::glfwGetKey(self.window.window_ptr(),glfw::ffi::GLFW_KEY_F11 as std::os::raw::c_int);
            if pressed && self.windinfo.is_fullscreen == true && !self.windinfo.is_pressed
            {
                
                glfw::ffi::glfwSetWindowMonitor(self.window.window_ptr(),std::ptr::null_mut(), self.windinfo.oldpositions.0, self.windinfo.oldpositions.1 , self.windinfo.olddimensions.0, self.windinfo.olddimensions.1, 0);
                self.windinfo.is_fullscreen = false;
  
            }
            else {if pressed && self.windinfo.is_fullscreen == false && !self.windinfo.is_pressed {

                let monitor = glfw::ffi::glfwGetPrimaryMonitor();
                let mode= glfw::ffi::glfwGetVideoMode(monitor);
                if !is_wayland
                {
                    let mut xpos:std::os::raw::c_int = 0;
                    let mut ypos:std::os::raw::c_int = 0;
                    glfw::ffi::glfwGetWindowPos(self.window.window_ptr(), &mut xpos,&mut ypos);
                    self.windinfo.oldpositions = (xpos,ypos);
                }
                else {
                    self.windinfo.oldpositions = (0,0);
                }
                let mut width:std::os::raw::c_int = 0;
                let mut height:std::os::raw::c_int = 0;
                glfw::ffi::glfwGetWindowSize(self.window.window_ptr(), &mut width, &mut height);
                self.windinfo.olddimensions = (width,height);
                glfw::ffi::glfwSetWindowMonitor(self.window.window_ptr(), monitor, 0, 0, (*mode).width, (*mode).height, (*mode).refreshRate);
                self.windinfo.is_fullscreen = true;
     
            }}
            
            self.windinfo.is_pressed = pressed;
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
            gl::Viewport(0, 0, self.windinfo.frame_buffer.0, self.windinfo.frame_buffer.1);
        }
        
        
        
    }
    pub fn end_drawing(&mut self)
    {
        self.window.swap_buffers();  
    }
}
impl Drop for WindowInstance {
    fn drop(&mut self) {
        
        unsafe {
            if !self.windinfo.is_primary
            {
                glfw::ffi::glfwDestroyWindow(self.window.window_ptr());
            }
            
        }
    }
}
