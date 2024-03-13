
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::RECT;

unsafe extern "system" fn show_window_info(hwnd: HWND, l_param: LPARAM) -> BOOL {
    // if IsWindowVisible(hwnd).as_bool() {
    let mut buffer: [u16; 512] = [0; 512];
    let mut title = String::new();
    let mut class_name = String::new();
    
    if IsWindowVisible(hwnd).as_bool() {
        let result = GetWindowTextW(hwnd, &mut buffer);
        title = String::from_utf16_lossy(&buffer);

        if result == 0 {
            return BOOL::from(true); // continue enumeration
        }

        GetClassNameW(hwnd, &mut buffer);
        class_name = String::from_utf16_lossy(&buffer);

        if class_name.contains("IME") || class_name.contains("CoreWindow") {
            return BOOL::from(true); // continue enumeration
        }

        let mut rect: RECT = RECT::default();
        GetWindowRect(hwnd, &mut rect);
        // let mut rect: RECT = RECT::default();
        // GetWindowRect(hwnd, &mut rect);

        let is_minimized = IsIconic(hwnd).as_bool();
        let is_maximized = IsZoomed(hwnd).as_bool();

        println!("Title: {}\nClass: {}\nRect: {:?}\nMinimized: {}\nMaximized: {}\n", title.trim(), class_name.trim(), rect, is_minimized, is_maximized);
    }
    return BOOL::from(true);
}

fn main() {
    unsafe {
        EnumWindows(Some(show_window_info), LPARAM(0));
    }

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
