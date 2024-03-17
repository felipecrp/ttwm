mod api;
mod container;
mod window;


use std::{thread, time};

use api::win::WinWindowManager;
use container::Workspace;
use window::WindowManager;

fn main() {
    let mut workspace = Workspace::new();

    let window_manager = WinWindowManager::new();
    let windows = window_manager.get_windows();
    for window in windows {
        println!("{}", window.get_name());
        workspace.add_window(window);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
