




use glfw::Glfw;
use glfw::{Context, GlfwReceiver,WindowEvent};



#[derive(Clone, Copy,PartialEq)]
pub struct  WindowHandle
{
    pub id:u32,
    pub generation:u32,
    pub is_primary:bool
}

pub struct Window {
    pub title:String,
    pub dimensions:(u32,u32),
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
    current_time:f64,
    pub frame_buffer:(i32,i32),
    windinfo:WindowInfo,
    is_primary:bool,
}
pub struct WindowInfo
{
    width:i32,
    height:i32,
    is_fullscreen:bool,
    is_pressed:bool,

}
impl WindowStorage
{
    pub fn new() -> WindowStorage
    {
        WindowStorage { handles: Vec::new(), windows: Vec::new(), window_to_index: Vec::new() }
    }
    pub fn get_window(&self,handle:&WindowHandle,fail_on_errors:bool) -> Result<&WindowInstance,&str>
    {
        if !fail_on_errors
        {
            if let Some(index) = self.window_to_index[handle.id as usize]
            {
                if self.handles[index] == *handle
                {
                    return Ok(&self.windows[index]);
                }
                return Err("invalid handle may be expired");
            }
            return Err("no window found");
        }
        else {
            if let Some(index) = self.window_to_index[handle.id as usize]
            {
                if self.handles[index] == *handle
                {
                    return Ok(&self.windows[index]);
                }
                panic!("handle is expired");
            }
            panic!("failed to get index");
        }
    }
    pub fn get_mut_window(&mut self,handle:&WindowHandle,fail_on_errors:bool) -> Result<&mut WindowInstance,&str>
    {
        if !fail_on_errors
        {
            if let Some(index) = self.window_to_index[handle.id as usize]
            {
                if self.handles[index] == *handle
                {
                    return Ok(&mut self.windows[index]);
                }
                return Err("invalid handle may be expired");
            }
            return Err("no window found");
        }
        else {
            if let Some(index) = self.window_to_index[handle.id as usize]
            {
                if self.handles[index] == *handle
                {
                    return Ok(&mut self.windows[index]);
                }
                panic!("handle is expired");
            }
            panic!("failed to get index");
        }
    }
           
    pub fn should_close(&mut self) ->bool
    {
        if self.windows.len() > 0
        {
            true
        }
        else {
            false
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
        let mut mainwind = WindowInstance::create_primary(runtime, 1, 1, "mainwind");
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
            _mainwindow = WindowInstance::create_shared(windowdata.dimensions.0, windowdata.dimensions.1, &windowdata.title, runtime,&wind);
        }
        else {
            if !windowdata.is_primary && windowdata.windowref.is_some()
            {
                let wind = self.storage.get_mut_window(&windowdata.windowref.unwrap(),true).unwrap();
                _mainwindow = WindowInstance::create_shared(windowdata.dimensions.0, windowdata.dimensions.1, &windowdata.title, runtime,wind);
            }
            else{
                if windowdata.is_primary 
                {
                    _mainwindow = WindowInstance::create_primary(runtime,windowdata.dimensions.0, windowdata.dimensions.1, &windowdata.title);
            
                }
                else
                {
                    panic!("failed to create window missing windowdata")
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
    pub fn update_events(&mut self)
    {
        
        let mut remove_queue: Vec<WindowHandle> = Vec::new();
        for i in 0..self.storage.windows.len()
        {   
            let wind = &mut self.storage.windows[i];
            
            wind.toggle_fullscreen();
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
    pub fn create_window(width:u32,height:u32,window_title:&str,runtime:&mut Glfw,primarywindow:&WindowInstance) -> WindowInstance
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
                return WindowInstance { window, events: (_events), delta_time:0.0,last_time:0.0,current_time:0.0,frame_buffer:(width,height),is_primary:false,windinfo:WindowInfo{width ,height,is_fullscreen:false,is_pressed:false}};    
            }
            else {
                panic!("failed to create shared window");
            }
    }
    pub fn create_primary(runtime:&mut Glfw,width:u32,height:u32,window_title:&str) -> WindowInstance
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

        WindowInstance  { 
            window, 
            events: (_events), 
            delta_time:0.0,
            last_time:0.0,
            current_time:0.0,
            frame_buffer:(width,height),
            windinfo:WindowInfo{width ,height,is_fullscreen:false,is_pressed:false},
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
    pub fn create_shared(width:u32,height:u32,window_title:&str,runtime:&mut Glfw,primarywindow:&WindowInstance) -> WindowInstance
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
                return WindowInstance{ window, events: (_events), delta_time:0.0,last_time:0.0,current_time:0.0,frame_buffer:(width,height),is_primary:false,windinfo:WindowInfo{width ,height,is_fullscreen:false,is_pressed:false}};    
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
            if pressed && self.windinfo.is_fullscreen == true && !self.windinfo.is_pressed
            {
                glfw::ffi::glfwSetWindowMonitor(self.window.window_ptr(),std::ptr::null_mut(), (*mode).width-self.windinfo.width / 2, (*mode).height-self.windinfo.height / 2, self.windinfo.width, self.windinfo.height, 0);
                self.windinfo.is_fullscreen = false;
  
            }
            else {if pressed && self.windinfo.is_fullscreen == false && !self.windinfo.is_pressed {
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
            gl::Viewport(0, 0, self.frame_buffer.0, self.frame_buffer.1);
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
            if !self.is_primary
            {
                glfw::ffi::glfwDestroyWindow(self.window.window_ptr());
            }
            
        }
    }
}
