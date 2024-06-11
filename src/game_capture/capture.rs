extern crate winapi;

use std::ffi::CStr;
use std::ptr::null_mut;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcesses, GetModuleFileNameExA};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub fn return_game_name() -> String{    
    let mut game = String::new();      
    game.push_str("");
    list_games();
    game
}
//this will be able to create a list of all running applications and set them an id
fn list_games() {
    let mut process_ids: [u32; 1024] = [0; 1024];
    let mut bytes_returned: u32 = 0;

    unsafe {
        if EnumProcesses(
            process_ids.as_mut_ptr(),
            std::mem::size_of_val(&process_ids) as u32,
            &mut bytes_returned,
        ) == 0
        {
            println!("Failed to enumerate processes");
            return;
        }

        let num_processes = bytes_returned / std::mem::size_of::<u32>() as u32;

        // Add new keywords
        let game_keywords = vec![
            "steam", "epic", "ldplayer","genshin","star rail","opera"
        ];
        let game_executable_names = vec![
            "steam.exe",
            "starrail.exe",
            "opera.exe",                                  
        ];

        for i in 0..num_processes as usize {
            let process_id = process_ids[i];
            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                0,
                process_id,
            );
        
            if process_handle.is_null() {
                continue;
            }
        
            let mut process_path: [i8; 260] = [0; 260];
        
            if GetModuleFileNameExA(
                process_handle,
                null_mut(),
                process_path.as_mut_ptr(),
                process_path.len() as u32,
            ) > 0
            {
                let path_cstr = CStr::from_ptr(process_path.as_ptr());
                if let Ok(path_str) = path_cstr.to_str() {
                    let process_path = path_str.to_lowercase();
                    // Check if the process path contains any of the game executable names
                    if game_keywords.iter().any(|&keyword| process_path.contains(keyword)) { 
                        if game_executable_names.iter().any(|&_name| process_path.contains("star rail\\launcher.exe")){
                            println!("Star Rail Launcher detected: PID {} - {}", process_id, path_str);
                        }                       
                        else if game_executable_names.iter().any(|&name| process_path.contains(name)){
                            println!("Game exe detected: PID {} - {}", process_id, path_str);
                        }                       
                        else{
                            println!("Non exe detected better to ignore: PID {} - {}", process_id, path_str);
                        }
                    }
                }
            }        
            CloseHandle(process_handle);
        }
    }
}

