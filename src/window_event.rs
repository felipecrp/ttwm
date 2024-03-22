
use crate::{desktop::Desktop, window::Window};


#[derive(Debug)]
pub struct WindowEventListener {
    // desktop: Box<Desktop>
}

impl WindowEventListener {
    fn on_event(&self) -> () {
        // WINDOW_EVENT_LISTENER.get().unwrap().on_create(window);
    }
    
    pub fn new() -> Self {
        Self {}
    }
}
