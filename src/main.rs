use game_capture::capture::return_game_name;
use gui::gui::create_gui;

mod gui;
mod game_capture;
mod games;
fn main() {        
    return_game_name();    
    
    let x_positions = [100,250,400];
    let y_positions = [100,100,100];
    let widths =[100,100,100];
    let heights =[50,50,50];

    let buttons = &[        
        (x_positions[0], y_positions[0], widths[0], heights[0], "Arknights"),
        (x_positions[1], y_positions[1], widths[1], heights[1], "FGO"),
        (x_positions[2], y_positions[2], widths[2], heights[2], "Overwatch"),
        // Add more buttons as needed
    ];
    create_gui(buttons);    
}
