use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use serde_json::Value;
use crate::games::overwatch::movement;

pub fn screen_capture() {
    // Spawn the Python process
    println!("starting capture");
    let mut child = Command::new("python")
        .arg("screen_capture.py")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn Python process");

    // Ensure we have a handle to the stdout of the child process
    let stdout = child.stdout.take().expect("Failed to open stdout");

    // Read data from the stdout in the main thread
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        match line {
            Ok(data) => {                
                process_data(&data);
            }
            Err(e) => eprintln!("Failed to read line from pipe: {}", e),
        }
    }

    // Wait for the child process to exit
    let _ = child.wait().expect("Python process wasn't running");
}

fn process_data(data: &str) {
    // Parse the JSON data
    match serde_json::from_str::<Value>(data) {
        Ok(json_data) => {
            if let Some(movement_detected) = json_data["movement_detected"].as_bool() {
                //value is always false
                if movement_detected{
                    println!("Movement Detected!");
                    movement();                                                        
                }
                else{
                    println!("No Movement Detected!");
                }
            }
        }
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
}