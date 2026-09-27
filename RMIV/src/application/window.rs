use std::io::SeekFrom::Current;

use glfw::{Context, GlfwReceiver, WindowEvent};

use crate::application::app::App;
pub struct Window 
{
    pub window:glfw::PWindow,
    pub events:GlfwReceiver<(f64,WindowEvent)>,
    pub delta_time:f32,
    last_time:f64,
    current_time:f64,
}

impl Window
{
    pub fn create(glfw:&mut glfw::Glfw,width:u32,height:u32,window_title:&str) -> Window
    {
        let (mut window,_events) = glfw.create_window(width, height, window_title, glfw::WindowMode::Windowed).expect("failed to init window");
        

        Window { 
            window, 
            events: (_events), 
            delta_time:0.0,
            last_time:0.0,
            current_time:0.0,
        }
    }

    pub fn delta_time(&mut self,current_time:f64) ->f32
    {
        self.current_time = current_time;
        self.delta_time = (self.current_time - self.last_time) as f32;
        self.last_time = self.current_time;
        return self.delta_time;
    }

   
}