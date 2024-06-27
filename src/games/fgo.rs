use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;
use serde_json::Value;
use enigo::{        
    Button,
    Direction::Click,
    Enigo, Mouse, Settings,   
    Coordinate::Abs,
};

use crate::game_capture::window_capture::WindowInfo;

pub fn start_fgo() -> bool {    
    //wondershare MirrorGo is the app i use to screen share my phone to my monitor for fgo
    //this will be the application that fgo is running on like ldplayer or bluestacks
    //the title value (the first value) is the name of the application 
    //the class_name (the second value) is the name of the name of the window value like Chrome_WidgetWin_1 for vs code 
    let window_info = WindowInfo::focus_and_fullscreen_window(Some("MirrorGo"), Some("MirrorGo"));        
    if window_info.hwnd.is_none() {
        return true;
    }  
    let _result = run_character_skills();  
    false
}
fn _process_starting_characters(){
    //this will look at the current line up of characters that you are taking into the battle
    //it will process what the code will need to do for a 3 turn farm using the current line up
}
fn run_character_skills() -> Result<(), Box<dyn std::error::Error>>{
    let mut skill = 1;
    let template_paths = vec![
        //wave 1
        "src/games/fgo_skill_images/koyan_skill_3.png",
        "src/games/fgo_skill_images/give_to_dps.png",
        "src/games/fgo_skill_images/koyan_skill_2.png",
        "src/games/fgo_skill_images/give_to_dps.png",

        "src/games/fgo_skill_images/koyan_skill_3.png",
        "src/games/fgo_skill_images/give_to_dps.png", 
        "src/games/fgo_skill_images/koyan_skill_2.png",
        "src/games/fgo_skill_images/give_to_dps.png",

        "src/games/fgo_skill_images/melu_skill_1.png",         
        "src/games/fgo_skill_images/melu_skill_2.png", 
        "src/games/fgo_skill_images/melu_skill_3.png",         
        //wave 2
        "src/games/fgo_skill_images/koyan_skill_1.png",
        "src/games/fgo_skill_images/give_to_dps.png",

        "src/games/fgo_skill_images/koyan_skill_1.png",
        "src/games/fgo_skill_images/give_to_dps.png",

        "src/games/fgo_skill_images/melu_skill_3.png",
        //wave 3
        "src/games/fgo_skill_images/master_skill.png",
        "src/games/fgo_skill_images/master_skill_3.png",

        "src/games/fgo_skill_images/oberon.png",
        "src/games/fgo_skill_images/koyan.png",
        "src/games/fgo_skill_images/replace.png",

        "src/games/fgo_skill_images/oberon_skill_1.png",
        "src/games/fgo_skill_images/oberon_skill_2.png",
        "src/games/fgo_skill_images/give_to_dps.png",
        "src/games/fgo_skill_images/oberon_skill_3.png",
        "src/games/fgo_skill_images/give_to_dps.png",

        "src/games/fgo_skill_images/melu_skill_1.png",
    ];    
    for template_path in template_paths {
        
        let output = Command::new("python")
            .arg("src/games/skill_usage.py")
            .arg(template_path)
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| "Failed to capture standard output")?;

        let reader = io::BufReader::new(output);
        for line in reader.lines() {
            let line = line?;
            let json: Value = serde_json::from_str(&line)?;

            match json["status"].as_str() {
                Some("success") => {
                    let x = json["position"][0].as_i64().unwrap_or(-1);
                    let y = json["position"][1].as_i64().unwrap_or(-1);
                    println!("Image found at position: ({}, {})", x, y);

                    if x != -1 && y != -1 {
                        if skill == 1{
                            move_mouse_to_position(x, y);
                            thread::sleep(Duration::from_secs(1)); 
                            skill = 2;
                        }
                        else if skill == 2{
                            move_mouse_to_position(x+200, y+200);
                            thread::sleep(Duration::from_secs(1)); 
                            mouse_click();
                            skill = 1;
                        }                       
                    }                    
                },
                Some("error") => {
                    let message = json["message"].as_str().unwrap_or("Unknown error");
                    println!("Error: {}", message);
                },
                _ => {
                    println!("Unexpected output: {}", line);
                }
            }
        }
    }
    Ok(())
}

fn move_mouse_to_position(x: i64, y: i64) {    
    let mut enigo = Enigo::new(&Settings::default()).unwrap();    
    enigo.move_mouse(x as i32, y as i32,Abs).unwrap();    
    println!("Mouse moved to: ({}, {})", x, y);
    enigo.button(Button::Left, Click).unwrap();    
}
fn mouse_click(){
    let mut enigo = Enigo::new(&Settings::default()).unwrap(); 
    enigo.button(Button::Left, Click).unwrap(); 
}
