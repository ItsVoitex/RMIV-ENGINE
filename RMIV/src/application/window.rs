
use glfw::{GlfwReceiver, WindowEvent};

use crate::{application::App};

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
    //creates a window using glfw 
    pub fn create(app:&mut App,width:u32,height:u32,window_title:&str) -> Window
    {
        if app.window.len() > 0 && let Some(ptr) = app.window.values_mut().next() && let Some((window,_events)) =  ptr.window.create_shared(width, height, window_title, glfw::WindowMode::Windowed)
        {
            print!("created shared context");
            return Window { window, events: (_events), delta_time:0.0, last_time:0.0,current_time:0.0,};
            
        }
        let (window,_events) = app.glfw.create_window(width, height, window_title, glfw::WindowMode::Windowed).expect("failed to window");
        

        Window { 
            window, 
            events: (_events), 
            delta_time:0.0,
            last_time:0.0,
            current_time:0.0,
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

   
}