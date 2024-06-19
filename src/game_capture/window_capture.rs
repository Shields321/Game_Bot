extern crate winapi;

use std::ffi::CString;
use winapi::shared::minwindef::LPARAM;
use winapi::um::winuser::{EnumWindows, GetClassNameA, GetWindowTextA, ShowWindow, SW_MAXIMIZE, SetForegroundWindow};
use winapi::shared::windef::HWND;

pub struct WindowInfo<'a> {
    pub title: Option<&'a str>,
    pub class_name: Option<&'a str>,
    pub hwnd: Option<HWND>, // Store the found window handle here
}

impl<'a> WindowInfo<'a> {
    pub fn focus_and_fullscreen_window(title: Option<&'a str>, class_name: Option<&'a str>) {
        let mut info = WindowInfo { title, class_name, hwnd: None };

        unsafe {
            // First attempt: Find window by title
            EnumWindows(Some(Self::enum_windows_callback_title), &mut info as *mut WindowInfo as LPARAM);
            
            // If not found by title, attempt to find by class name
            if info.hwnd.is_none() && info.class_name.is_some() {
                EnumWindows(Some(Self::enum_windows_callback_class), &mut info as *mut WindowInfo as LPARAM);
            }
        }

        if let Some(hwnd) = info.hwnd {
            unsafe {
                // Maximize the window
                ShowWindow(hwnd, SW_MAXIMIZE);
                // Bring the window to the foreground and focus it
                SetForegroundWindow(hwnd);
            }
            println!("Found and maximized window.");
        } else {
            println!("Window not found.");
        }
    }

    unsafe extern "system" fn enum_windows_callback_title(hwnd: HWND, info_ptr: LPARAM) -> i32 {
        let info = &mut *(info_ptr as *mut WindowInfo);

        if let Some(title) = info.title {
            const MAX_TITLE_LENGTH: usize = 256;
            let mut window_title = vec![0u8; MAX_TITLE_LENGTH];
            // Get the window title
            let title_len = GetWindowTextA(hwnd, window_title.as_mut_ptr() as *mut _, MAX_TITLE_LENGTH as i32);
            if title_len > 0 {
                let window_title = CString::from_vec_unchecked(window_title);
                let window_title_str = window_title.to_string_lossy();            
                // Check if the window title matches the desired title
                if window_title_str.contains(title) {
                    info.hwnd = Some(hwnd);
                    return 0; // Stop enumeration
                }
            }
        }

        1 // Continue enumeration
    }

    unsafe extern "system" fn enum_windows_callback_class(hwnd: HWND, info_ptr: LPARAM) -> i32 {
        let info = &mut *(info_ptr as *mut WindowInfo);

        if let Some(class_name) = info.class_name {
            const MAX_CLASS_NAME_LENGTH: usize = 256;
            let mut class_name_buffer = vec![0u8; MAX_CLASS_NAME_LENGTH];        
            // Get the window class name
            let class_len = GetClassNameA(hwnd, class_name_buffer.as_mut_ptr() as *mut _, MAX_CLASS_NAME_LENGTH as i32);
            if class_len > 0 {
                let class_name_str = CString::from_vec_unchecked(class_name_buffer);
                let class_name_str = class_name_str.to_string_lossy();            
                // Check if the window class name matches the desired class name
                if class_name_str.contains(class_name) {
                    info.hwnd = Some(hwnd);
                    return 0; // Stop enumeration
                }
            }
        }

        1 // Continue enumeration
    }
}
