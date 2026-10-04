
use glfw::{Context, GlfwReceiver, WindowEvent};

use crate::application::{App};

pub struct Window 
{
    pub window:glfw::PWindow,
    pub events:GlfwReceiver<(f64,WindowEvent)>,
    pub delta_time:f32,
    last_time:f64,
    current_time:f64,
    pub frame_buffer:(i32,i32),
    is_primary:bool,
}

impl Window
{
    //creates a window using glfw 
    pub fn create(app:&mut App,width:u32,height:u32,window_title:&str) -> Window
    {

        let (window,_events) = app.glfw.create_window(width, height, window_title, glfw::WindowMode::Windowed).expect("failed to create primary window");

        Window { 
            window, 
            events: (_events), 
            delta_time:0.0,
            last_time:0.0,
            current_time:0.0,
            frame_buffer:(1920,1080),
            is_primary:false
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
    pub fn create_shared(app:&mut App,width:u32,height:u32,window_title:&str) -> Window
    {
        
         if let Some((window,_events)) =  app.window.window.create_shared(width, height, window_title, glfw::WindowMode::Windowed)
            {
                return Window { window, events: (_events), delta_time:0.0,last_time:0.0,current_time:0.0,frame_buffer:(1920,1080),is_primary:false};    
            }
            else {
                panic!("failed to create shared window");
            }
        
    }
    pub fn basic_window(window:glfw::PWindow,events:GlfwReceiver<(f64,WindowEvent)>) ->Window
    {
        

        Window { 
            window, 
            events: (events), 
            delta_time:0.0,
            last_time:0.0,
            current_time:0.0,
            frame_buffer:(0,0),
            is_primary:true
        }
    }
    pub fn process_events(&mut self) -> bool
    {
        let mut should_remove = false;
    
        for (_,events) in glfw::flush_messages(&self.events)
        {
            match events {
                glfw::WindowEvent::Key(glfw::Key::Escape,_ ,glfw::Action::Press ,_ ) =>
                {
                    print!("doing");
                    self.window.set_should_close(true);
                    should_remove = true;
                },
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