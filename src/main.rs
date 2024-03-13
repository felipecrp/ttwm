pub mod node;

use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

use node::*;

pub struct App {
    name: String,
    window: HWND
}

fn main() {
    let cur_window;
    unsafe { 
        cur_window = GetForegroundWindow ();
    }

    let cur_app = App {
        name: String::from(""),
        window: cur_window,
    };

    unsafe {
        let _result = MoveWindow(cur_app.window, 10, 10, 700, 700, true); 
    }
}
