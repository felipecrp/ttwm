mod api;
mod container;
mod window;

use crate::window::Window;


fn main() {
    let windows = api::get_all_windows();    
    for window in windows {
        println!("{}", crate::api::win::Window::from(window).get_name())
    }
}
