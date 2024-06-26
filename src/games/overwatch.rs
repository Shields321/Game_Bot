use enigo::{
    Button,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
    {Axis::Horizontal, Axis::Vertical},
    {Coordinate::Abs, Coordinate::Rel},
};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref ENIGO: Arc<Mutex<Enigo>> = Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap())); 
}
pub fn movement(){  
    //sleep before doing anything to let the user go back into the game
    thread::sleep(Duration::from_secs(5));    
    //key w is pressed
    let enigo_clone = Arc::clone(&ENIGO);
    thread::spawn(move || turn_screen(enigo_clone));

    let mut enigo = ENIGO.lock().unwrap();
    //TODO this loops indefinitely change to allow the user to exit the loop    
    
    key_press('w',&mut *enigo);   
    thread::sleep(Duration::from_secs(1));    
    key_release('w',&mut *enigo); 
            
    key_press(' ',&mut *enigo);   
    thread::sleep(Duration::from_secs(1));
    key_release(' ',&mut *enigo); 

    left_click(&mut *enigo);
    thread::sleep(Duration::from_secs(1));    
       
}
fn key_press(key: char, enigo: &mut Enigo) {    
    enigo.key(Key::Unicode(key), Press).unwrap(); 
}
fn key_release(key: char, enigo: &mut Enigo) {    
    enigo.key(Key::Unicode(key), Release).unwrap();
}
fn left_click(enigo: &mut Enigo) {
    enigo.button(Button::Left, Click).unwrap();  
}
fn turn_screen(enigo: Arc<Mutex<Enigo>>){
    let mut enigo = enigo.lock().unwrap();
    for _ in 0..5 {
        enigo.move_mouse(100, 0, Rel).unwrap();
        thread::sleep(Duration::from_millis(500)); 
    }
}