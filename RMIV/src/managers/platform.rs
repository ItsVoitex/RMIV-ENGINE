use crate::{managers::window::{WindowHandle, WindowManager}};

use glfw::{Context, fail_on_errors};
use std::mem;
use crate::prelude::*;

pub struct Platform
{
    pub runtime:glfw::Glfw,
    pub windowmanager:WindowManager,
    is_wayland:bool
}


impl Platform
{
    pub fn new() ->Platform
    {
        let mut runtime = glfw::init(fail_on_errors).unwrap();
        let platform = runtime.get_platform();
        let is_wayland = platform == glfw::Platform::Wayland;
        let windowmanager = WindowManager::new(&mut runtime);
        Platform { runtime, windowmanager ,is_wayland}

    }
    pub fn update_events(&mut self)
    {
        self.runtime.poll_events();
        self.windowmanager.update_events(&mut self.runtime,&self.is_wayland);
    }
    pub fn delta_time(&mut self,window_handle:&WindowHandle) -> f32
    {
        let window = self.windowmanager.get_mut(window_handle,false);
        let window = match window{
            Ok(wind) => wind,
            Err(error) => {println!("{} unable to calculate delta time",error);return 0.0},
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
                Err(error) => {println!("{}",error);return None},
            };
            std::mem::transmute(glfw::ffi::glfwGetKey(window.window.window_ptr(), KeyCode as std::os::raw::c_int)) 
        }     
    }
    pub fn get_cursor_position(&self,window_handle:&WindowHandle,camera:&mut Camera) -> glam::Vec2
    {
        unsafe { 
            let window = 
            match self.windowmanager.get(window_handle,false){
                Ok(wind) => wind,
                Err(error) => {println!("{}",error);return vec2(0.0, 0.0)},
            };
            let mut xpos = 0.0;
            let mut ypos = 0.0;
            glfw::ffi::glfwGetCursorPos(window.window.window_ptr(),&mut xpos,&mut ypos);

            let xpos = 2.0 * (xpos/window.windinfo.frame_buffer.0 as f64) - 1.0;
            let ypos = 1.0 - 2.0 * (ypos/window.windinfo.frame_buffer.1 as f64);

            let iproj = camera.proj.inverse();
            let iview = camera.view.inverse();
            let imodel = camera.model.inverse();

            let mut cords = vec4(xpos as f32, ypos as f32, 0.0,0.0);

            cords = iproj * iview * imodel * cords;
            return vec2(cords.x, cords.y);
        }
    }
    #[allow(non_snake_case)]
    fn get_mouse(&self,window_handle:&WindowHandle, MouseButton: MouseButton) -> Option<Action>
    {
        unsafe { 
            let window = 
            match self.windowmanager.get(window_handle,false){
                Ok(wind) => wind,
                Err(error) => {println!("{}",error);return None},
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

