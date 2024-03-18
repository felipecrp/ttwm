
use crate::{desktop::Desktop, window::Window};


pub struct WindowEventListener {
    // desktop: Box<Desktop>
}

impl WindowEventListener {
    fn on_create(&self, window: &dyn Window) -> () {
        // WINDOW_EVENT_LISTENER.get().unwrap().on_create(window);
    }
}
