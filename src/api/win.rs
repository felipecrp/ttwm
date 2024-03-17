
use std::collections::HashMap;
use std::mem::zeroed;

use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};

use crate::container::WindowShape;
use crate::window::{Window, WindowManager};

pub struct WinWindow {
    window_handler: WindowHandler,
    shape: WindowShape
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
    
    fn get_shape(&self) -> WindowShape {
        todo!()
    }
    
    fn set_shape(&self, _shape: WindowShape) -> () {
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
            shape: WindowShape {
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

    fn get_shape(&self) -> WindowShape {
        let mut rect: RECT = RECT::default();
        unsafe { GetWindowRect(self.0, &mut rect).unwrap() };
        WindowShape {
            x: rect.left,
            y: rect.top,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top
        }
    }
    
    fn mve(&self, geo: (i32, i32, i32, i32)) -> () {
        unsafe { MoveWindow(self.0, geo.0, geo.1, geo.2, geo.3, true) }; 
    }
}

impl From<HWND> for WindowHandler {
    fn from(hwnd: HWND) -> Self {
        WindowHandler(hwnd)
    }
}


pub struct WinWindowManager {
    pub(crate) windows: HashMap<WindowHandler, WinWindow>
}

impl WinWindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::<WindowHandler, WinWindow>::new()
        }
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
    unsafe { EnumWindows(
        Some(enum_windows_proc),
        LPARAM(&mut window_handles as *mut Vec<WindowHandler> as isize)
    ).unwrap() };
    window_handles.into_iter()
        .filter(|window_handle| window_handle.is_window())
        .collect()
}

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let handles = unsafe { &mut *(lparam.0 as *mut Vec<WindowHandler>) };
    handles.push(WindowHandler(hwnd));
    BOOL::from(true)
}

// impl From<HWND> for WindowHandler {
//     fn from(hwnd: HWND) -> Self {
//         unsafe {
//             let mut buffer: [u16; 512] = [0; 512];
//             let name_len = GetWindowTextW(hwnd, &mut buffer) as usize;
//             let name = String::from_utf16_lossy(&buffer[0..name_len]);
//             let is_visible = IsWindowVisible(hwnd).as_bool();
//             let is_minimized = IsIconic(hwnd).as_bool();
//             let is_maximized = IsZoomed(hwnd).as_bool();
            
//             let mut rect: RECT = RECT::default();
//             GetWindowRect(hwnd, &mut rect).unwrap();

//             let mut window_info: WINDOWINFO = zeroed();
//             window_info.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
//             GetWindowInfo(hwnd, &mut window_info).unwrap();
        
//             let window_style: WINDOW_STYLE = window_info.dwStyle;
//             let is_popup = window_style & WS_POPUP == WS_POPUP;


//             WindowContainer {
//                 name: name, 
//                 size: Dimension { 
//                     width: rect.right - rect.left, 
//                     height: rect.bottom - rect.top 
//                 } 
//             }
//         }
//     }
// }


// fn is_window(hwnd: HWND) -> bool {

//     if IsWindowVisible(hwnd).as_bool() {
//         return false;
//     }

//     true

//     let mut buffer: [u16; 512] = [0; 512];
//     let mut title = String::new();
//     let mut class_name = String::new();
    
//     if IsWindowVisible(hwnd).as_bool() {
//         let result = GetWindowTextW(hwnd, &mut buffer);
//         title = String::from_utf16_lossy(&buffer);

//         if result == 0 {
//             return BOOL::from(true); // continue enumeration
//         }

//         GetClassNameW(hwnd, &mut buffer);
//         class_name = String::from_utf16_lossy(&buffer);

//         if class_name.contains("IME") || class_name.contains("CoreWindow") {
//             return BOOL::from(true); // continue enumeration
//         }

//         let mut rect: RECT = RECT::default();
//         GetWindowRect(hwnd, &mut rect);

//         let is_minimized = IsIconic(hwnd).as_bool();
//         let is_maximized = IsZoomed(hwnd).as_bool();


//         let mut window_info: WINDOWINFO = zeroed();
//         window_info.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
//         let result = GetWindowInfo(hwnd, &mut window_info);
//         if matches!(result, Result::Err(_)) {
//             return BOOL::from(true);
//         }


//         let window_style: WINDOW_STYLE = window_info.dwStyle;
//         let is_popup = window_style & WS_POPUP;

//         if is_popup == WS_POPUP {
//             return BOOL::from(true);
//         }

//         println!("Title: {}\nClass: {}\nRect: {:?}\nMinimized: {}\nMaximized: {}", title.trim(), class_name.trim(), rect, is_minimized, is_maximized);
//         println!("Info: {:?}", window_info);
//         println!("Popup: {:?}", is_popup);

//         println!();
//     }
//     return BOOL::from(true);
// } 
//     true
// }





// use std::mem::zeroed;

// use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};
// use windows::Win32::Foundation::LPARAM;
// use windows::Win32::Foundation::BOOL;
// use windows::Win32::Foundation::RECT;

// unsafe extern "system" fn show_window_info(hwnd: HWND, l_param: LPARAM) -> BOOL {
//     // if IsWindowVisible(hwnd).as_bool() {
//     let mut buffer: [u16; 512] = [0; 512];
//     let mut title = String::new();
//     let mut class_name = String::new();
    
//     if IsWindowVisible(hwnd).as_bool() {
//         let result = GetWindowTextW(hwnd, &mut buffer);
//         title = String::from_utf16_lossy(&buffer);

//         if result == 0 {
//             return BOOL::from(true); // continue enumeration
//         }

//         GetClassNameW(hwnd, &mut buffer);
//         class_name = String::from_utf16_lossy(&buffer);

//         if class_name.contains("IME") || class_name.contains("CoreWindow") {
//             return BOOL::from(true); // continue enumeration
//         }

//         let mut rect: RECT = RECT::default();
//         GetWindowRect(hwnd, &mut rect);

//         let is_minimized = IsIconic(hwnd).as_bool();
//         let is_maximized = IsZoomed(hwnd).as_bool();


//         let mut window_info: WINDOWINFO = zeroed();
//         window_info.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
//         let result = GetWindowInfo(hwnd, &mut window_info);
//         if matches!(result, Result::Err(_)) {
//             return BOOL::from(true);
//         }


//         let window_style: WINDOW_STYLE = window_info.dwStyle;
//         let is_popup = window_style & WS_POPUP;

//         if is_popup == WS_POPUP {
//             return BOOL::from(true);
//         }

//         println!("Title: {}\nClass: {}\nRect: {:?}\nMinimized: {}\nMaximized: {}", title.trim(), class_name.trim(), rect, is_minimized, is_maximized);
//         println!("Info: {:?}", window_info);
//         println!("Popup: {:?}", is_popup);

//         println!();
//     }
//     return BOOL::from(true);
// }

// fn main() {
//     container::test();
    // unsafe {
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     println!("Checking");
    //     EnumWindows(Some(show_window_info), LPARAM(0));
    // }

    //     let display_width = GetSystemMetrics(SM_CXSCREEN);
    //     let display_height = GetSystemMetrics(SM_CYSCREEN);
    //     print!("{display_width}x{display_height}");

    //     let mut buffer: [u16; 512] = [0; 512];
    //     let mut title = String::new();
    //     let mut class_name = String::new();

    //     let mut hwnd = GetTopWindow(None); 
    //     while hwnd != HWND::default() {
    //         if IsWindowVisible(hwnd).as_bool() {
    //             GetWindowTextW(hwnd, &mut buffer);
    //             title = String::from_utf16_lossy(&buffer);

    //             GetClassNameW(hwnd, &mut buffer);
    //             class_name = String::from_utf16_lossy(&buffer);

    //             // let mut rect: RECT = RECT::default();
    //             // GetWindowRect(hwnd, &mut rect);

    //             println!("Title: {}, Class: {}, Rect: {:?}\n", title.trim(), class_name.trim(), "");
    //         }
    //         hwnd = GetWindow(hwnd, GW_HWNDNEXT);
    //     }
    // }

    // let cur_window;
    // unsafe { 
    //     cur_window = GetForegroundWindow ();
    // }

    // let cur_app = App {
    //     name: String::from(""),
    //     window: cur_window,
    // };

    // unsafe {
    //     let _result = MoveWindow(cur_app.window, 10, 10, 700, 700, true); 
    // }

// }
