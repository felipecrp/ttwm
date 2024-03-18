
use std::collections::HashMap;
use std::mem::zeroed;
use std::sync::{Mutex, OnceLock};

use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::UI::Accessibility::*;

use crate::window_event::{WindowEventListener};
use crate::geometry::Shape;
use crate::window::{Window, WindowManager};

pub struct WinWindow {
    window_handler: WindowHandler,
    geometry: Shape
}

impl Window for WinWindow {
    fn get_name(&self) -> String {
        self.window_handler.get_name()
    }
    
    fn is_maximized(&self) -> bool {
        self.window_handler.is_maximized()
    }
    
    fn is_minimized(&self) -> bool {
        self.window_handler.is_minimized()
    }
    
    fn get_shape(&self) -> Shape {
        todo!()
    }
    
    fn set_shape(&self, _shape: Shape) -> () {
        todo!()
    }
    
    fn mve(&self, geo: (i32, i32, i32, i32)) -> () {
        self.window_handler.mve(geo);
    }
}

impl From<WindowHandler> for WinWindow {
    fn from(hwnd: WindowHandler) -> Self {
        let shape = hwnd.get_shape();
        WinWindow {
            window_handler: hwnd,
            geometry: Shape {
                x: shape.x,
                y: shape.y,
                width: shape.width,
                height: shape.height
            }
        }
    }
}


pub struct WindowHandler(HWND);

impl WindowHandler {
    fn get_name(&self) -> String {
        let mut buffer: [u16; 512] = [0; 512];
        let length = unsafe { GetWindowTextW(self.0, &mut buffer) } as usize;
        String::from_utf16_lossy(&buffer[0..length])
    }

    fn is_maximized(&self) -> bool {
        unsafe { IsZoomed(self.0).as_bool() }
    }

    fn is_minimized(&self) -> bool {
        unsafe { IsIconic(self.0).as_bool() }
    }

    // Return the window info 
    fn get_window_info(&self) -> WINDOWINFO {
        let mut window_info: WINDOWINFO = unsafe { zeroed() };
        window_info.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
        unsafe { GetWindowInfo(self.0, &mut window_info).unwrap() };
        window_info
    }

    // Return the window style
    fn get_window_style(&self) -> WINDOW_STYLE {
        self.get_window_info().dwStyle
    }

    // Return true if the window is a popup
    fn is_popup(&self) -> bool {
        self.get_window_style() & WS_POPUP == WS_POPUP
    }

    // Return true if the window is visible
    fn is_window_visible(&self) -> bool {
        unsafe { IsWindowVisible(self.0).as_bool() } 
    }

    /// Return true if the handle is a window
    fn is_window(&self) -> bool {
        self.is_window_visible() && !self.is_popup()
    }

    fn get_shape(&self) -> Shape {
        let mut rect: RECT = RECT::default();
        unsafe { GetWindowRect(self.0, &mut rect).unwrap() };
        Shape {
            x: rect.left,
            y: rect.top,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top
        }
    }
    
    fn mve(&self, geo: (i32, i32, i32, i32)) -> () {
        // unsafe { MoveWindow(self.0, geo.0, geo.1, geo.2, geo.3, true) }; 
        unsafe { let _ = SetWindowPos(
            self.0, HWND_TOP, geo.0, geo.1, geo.2, geo.3, 
            SWP_FRAMECHANGED|SWP_NOACTIVATE|SWP_NOCOPYBITS|SWP_NOSENDCHANGING|SWP_ASYNCWINDOWPOS
        ); }; 
    }
}

impl From<HWND> for WindowHandler {
    fn from(hwnd: HWND) -> Self {
        WindowHandler(hwnd)
    }
}

static WINDOW_EVENT_LISTENER: OnceLock<WindowEventListener> = OnceLock::new();
// static WINDOW_EVENT_LISTENER: Mutex<WindowEventListener> = Mutex::new();

pub struct WinWindowManager {
    pub windows: HashMap<WindowHandler, WinWindow>
}

impl WinWindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::<WindowHandler, WinWindow>::new()
        }
    }

    pub fn register(&self, window_event_listener: WindowEventListener) -> &WindowEventListener {
        let listener = WINDOW_EVENT_LISTENER.get_or_init(|| window_event_listener);
        listener
    }

    pub fn evt(&self, f: &dyn Fn(i32) -> ()) -> () {
    }
}

impl WindowManager for WinWindowManager {
    fn get_windows(&self) -> Vec<Box<dyn Window>> {
        let mut windows = Vec::<Box<dyn Window>>::new();

        for window_handle in get_all_windows() {
            let window = WinWindow::from(window_handle);
            windows.push(Box::new(window));
        }

        windows
    }
}

// Option<&WindowHandler>


// impl WindowManager for WinWindowManager {
//     fn get_windows(&self) -> Vec<Box<dyn Window>> {
//         let collect = get_all_windows().into_iter()
//             .filter(|window_handle| window_handle.is_window())
//             // .map(|window_handle| <dyn Window>::from(window_handle))
//             .collect();
//     }
// }

pub fn get_all_windows() -> Vec<WindowHandler> {
    let mut window_handles: Vec<WindowHandler> = Vec::new();
    unsafe { 
        EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut window_handles as *mut Vec<WindowHandler> as isize)
        ).unwrap()
    };
    window_handles.into_iter()
        .filter(|window_handle| window_handle.is_window())
        .collect()
}

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let handles = unsafe { &mut *(lparam.0 as *mut Vec<WindowHandler>) };
    handles.push(WindowHandler(hwnd));
    BOOL::from(true)
}

// use std::sync::{Mutex, OnceLock};


// pub struct WinEventRegister<'a>(Option<&'a WindowEventListener>);

// // pub static WINDOW_EVENT_REGISTER: Mutex<WinEventRegister> = Mutex::new(WinEventRegister(None)); 

// impl<'a> WinEventRegister<'a> {
//     pub fn register(&mut self, listener: Option<&'a WindowEventListener>) -> () {
//         unsafe { self.0 = listener };
//     }
// }


// pub struct WinWindowEventListener { }

// impl WinWindowEventListener {
//     pub fn new() -> Self {
//         let listener = Self {};
//         listener
//     }

//     pub fn register(&self) {
//         let listener = &self;
//         // unsafe { WINDOW_EVENT_REGISTER.register(Some(listener)) };
//     }
    
//     pub fn listen(&self, evt: i32) -> () {
//     }
// }

// impl WindowEventListener for WinWindowEventListener {
//     fn register(&self, callback: &WindowEventCallback) {
//         let event = unsafe { SetWinEventHook(
//             EVENT_SYSTEM_FOREGROUND, 
//             EVENT_SYSTEM_FOREGROUND, 
//             None,
//             Some(create_object_proc),
//             0, 0, 0
//         )};
//     }
// }

// unsafe extern "system" fn create_object_proc(
//     hwineventhook: HWINEVENTHOOK, 
//     event: u32, 
//     hwnd: HWND,
//     idobject: i32,
//     idchild: i32,
//     ideventthread: u32,
//     dwmseventtime: u32) {
    
//     println!("Event");
// }

