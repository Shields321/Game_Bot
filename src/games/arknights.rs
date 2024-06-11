use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use serde_json::Value;


pub fn screen_capture() {
    // Spawn the Python process
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
                println!("Received data from Python: {}", data);
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
            if let Some(detected_text) = json_data["detected_text"].as_str() {
                println!("Detected Text: {}", detected_text);

                // Pass the parsed data to the recording function
                record_monitor(detected_text.to_string());
            }
        }
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
}

fn record_monitor(detected_text: String) {
    // Process the received data as needed
    // For example, you can use the data to control the game recording logic
    println!("Recording game with Detected Text: {}", detected_text);
}