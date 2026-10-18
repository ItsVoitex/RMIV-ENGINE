
/*use crate::{managers::{window::WindowHandle, world::Component}, prelude::*};
pub struct Commands<'a>{
    pub app:&'a mut App<T>,
}
impl <'a>Commands<'a>
{
    pub fn new(app:&'a mut App<T>) ->Self
    {
        return Commands { app };
    }
    pub fn spawn<T:Component>(&mut self,components:T)
    {
        self.app.world.spawn(components);
    }
    pub fn create_window(&mut self,width:u32,height:u32,window_title:&str) -> WindowHandle
    {
        self.app.platform.windows.create_window(&mut self.app.platform.runtime,width,height,window_title)
    }
}

*/