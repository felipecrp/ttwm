mod api;
mod container;
mod window;

use api::win::WinWindowManager;
use window::WindowManager;

fn main() {
    let window_manager = WinWindowManager;
    let windows = window_manager.get_windows();
    for window in windows {
        println!("{}", window.get_name());
    }
}
