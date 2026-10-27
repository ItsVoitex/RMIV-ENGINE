use crate::{managers::window::{WindowHandle, WindowManager}};
use glfw::{Context, fail_on_errors};
use std::mem;
use crate::prelude::*;

pub struct Platform
{
    pub runtime:glfw::Glfw,
    pub windowmanager:WindowManager
}


impl Platform
{
    pub fn new() ->Platform
    {
        let mut runtime = glfw::init(fail_on_errors).unwrap();
        
        let windowmanager = WindowManager::new(&mut runtime);
        Platform { runtime, windowmanager }

    }
    pub fn update_events(&mut self)
    {
        self.runtime.poll_events();
        self.windowmanager.update_events(&mut self.runtime);
    }
    pub fn delta_time(&mut self,window_handle:&WindowHandle) -> f32
    {
        let window = self.windowmanager.get_mut(window_handle,false);
        let window = match window{
            Ok(wind) => wind,
            Err(error) => {println!("failed to get window due to {} unable to calculate delta time",error);return 0.0},
        };
        return window.delta_time(self.runtime.get_time())
    }
    pub fn close_window(&mut self,windowhandle:WindowHandle)
    {
        if let Ok(wind) = self.windowmanager.get_mut(&windowhandle,false)
        {
            wind.window.set_should_close(true);
        }

    }
    pub fn create_window(&mut self,windowdata:Window) -> WindowHandle
    {
        self.windowmanager.create_window(&mut self.runtime, windowdata)
    }
    
    
    #[allow(non_snake_case)]
    fn get_KeyCode(&self,window_handle:&WindowHandle, KeyCode: KeyCode) -> Option<Action> {
        unsafe 
        { 
            let window =  
            match self.windowmanager.get(window_handle,false){
                Ok(wind) => wind,
                Err(error) => {println!("failed to get window due to {}",error);return None},
            };
            std::mem::transmute(glfw::ffi::glfwGetKey(window.window.window_ptr(), KeyCode as std::os::raw::c_int)) 
        }     
    }
    #[allow(non_snake_case)]
    fn get_mouse(&self,window_handle:&WindowHandle, MouseButton: MouseButton) -> Option<Action>
    {
        unsafe { 
            let window = 
            match self.windowmanager.get(window_handle,false){
                Ok(wind) => wind,
                Err(error) => {println!("failed to get window due to {}",error);return None},
            };
            mem::transmute(glfw::ffi::glfwGetMouseButton(window.window.window_ptr(), MouseButton as std::os::raw::c_int)) 
        }
        
    }
    #[allow(non_snake_case)]
    pub fn mouse_is_pressed(&self,window_handle:&WindowHandle, MouseButton: MouseButton) -> bool
    {
        if (self.get_mouse(window_handle,MouseButton)) == Some(Action::Press)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    #[allow(non_snake_case)]
    pub fn mouse_is_released(&self,window_handle:&WindowHandle, MouseButton: MouseButton) -> bool
    {
        if (self.get_mouse(window_handle,MouseButton)) == Some(Action::Release)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    #[allow(non_snake_case)]
    pub fn mouse_is_repeated(&self,window_handle:&WindowHandle, MouseButton: MouseButton) -> bool
    {
        if (self.get_mouse(window_handle,MouseButton)) == Some(Action::Repeat)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    #[allow(non_snake_case)]
    pub fn Key_is_pressed(&self,window_handle:&WindowHandle, KeyCode: KeyCode) -> bool
    {
        if (self.get_KeyCode(window_handle,KeyCode)) == Some(Action::Press)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    #[allow(non_snake_case)]
    pub fn Key_is_released(&self,window_handle:&WindowHandle, KeyCode: KeyCode) -> bool
    {
        if (self.get_KeyCode(window_handle,KeyCode)) == Some(Action::Release)
        {
            return true;
        }
        else {
            return  false;
        }
    }
    #[allow(non_snake_case)]
    pub fn Key_is_repeated(&self,window_handle:&WindowHandle, KeyCode: KeyCode) -> bool
    {
        if (self.get_KeyCode(window_handle,KeyCode)) == Some(Action::Repeat)
        {
            return true;
        }
        else {
            return  false;
        }
    }
}

