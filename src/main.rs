use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use std::io;

use game_capture::capture::return_game_name;
use games::fgo::start_fgo;
use gui::gui::create_gui;
use game_capture::screen_capture::screen_capture;

mod gui;
mod game_capture;
mod games;

fn main() {
    return_game_name();  
          
    // Atomic flag to control the running state
    let running = Arc::new(AtomicBool::new(true));

    let x_positions = [100, 250, 400];
    let y_positions = [100, 100, 100];
    let widths = [100, 100, 100];
    let heights = [50, 50, 50];

    let buttons = &[
        (x_positions[0], y_positions[0], widths[0], heights[0], "Arknights"),
        (x_positions[1], y_positions[1], widths[1], heights[1], "FGO"),
        (x_positions[2], y_positions[2], widths[2], heights[2], "Overwatch"),        
    ];

    let mut game_selected = true;
    while game_selected {
        let game = create_gui(buttons);        
        if game.unwrap() == "Overwatch" {
            // Spawn the key press thread for Overwatch
            let screen_running = Arc::clone(&running);
            let screen_capture_handle = thread::spawn(move || {
                while screen_running.load(Ordering::SeqCst) {
                    screen_capture();
                    thread::sleep(Duration::from_millis(100));
                }
            });

            // Wait for the user to press Enter to stop
            println!("Press Enter to stop...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Set the running flag to false to stop the threads
            running.store(false, Ordering::SeqCst);

            // Wait for the key press thread to finish
            // Wait for the screen capture thread to finish
            match screen_capture_handle.join() {
                Ok(_) => println!("Screen capture finished"),
                Err(e) => eprintln!("Screen capture thread panicked: {:?}", e),
            }
            game_selected = false;

        } if game.unwrap() =="FGO"{
            game_selected = start_fgo();            
        }
        else {
            println!("Selected game: {}", game.unwrap());
            continue;
        }
    }    
}
