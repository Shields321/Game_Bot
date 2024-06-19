use crate::game_capture::window_capture::WindowInfo;

pub fn start_fgo(){    
    //wondershare MirrorGo is the app i use to screen share my phone to my monitor for fgo
    //this will be the application that fgo is running on like ldplayer or bluestacks
    //the title value (the first value) is the name of the application 
    //the class_name (the second value) is the name of the name of the window value like Chrome_WidgetWin_1 for vs code 
    WindowInfo::focus_and_fullscreen_window(Some("MirrorGo"), Some("MirrorGo"));           
}
