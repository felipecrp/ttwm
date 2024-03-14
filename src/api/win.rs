use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};
use windows::Win32::Foundation::{BOOL, LPARAM};

use crate::container;

pub fn get_all_windows() -> Vec<container::WindowContainer> {
    let windows: Vec<container::WindowContainer> = Vec::new();
    let mut hwnds: Vec<HWND> = Vec::new();

    unsafe { EnumWindows(
        Some(get_all_windows_hwnd),
        LPARAM(&mut hwnds as *mut Vec<HWND> as isize))
    };

    windows
}

extern "system" fn get_all_windows_hwnd(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // let mut hwnds: Vec<HWND> = lparam as
    let hwnds = lparam as Vec<HWND>;
    
    // &mut Vec<HWND> as &mut Vec<HWND>;?
    BOOL::from(true)
}


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

fn main() {
    container::test();
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

}
