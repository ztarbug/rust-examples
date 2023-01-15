use std::env;
use opencv::{
    prelude::*,
    videoio,
    highgui,
    imgcodecs
};

use opencv::core::Vector;

enum Function {
    Display,
    Save
}

impl Function {
    pub fn parse_function_string(s: &str) -> Result<Function, &'static str> {
        match s {
            "display" => Ok(Function::Display),
            "save" => Ok(Function::Save),
            _ => Err("couldn't parse command")
        }
    }
}

fn main() { 

    let params: Vec<String> = env::args().collect();
    match Function::parse_function_string(params.get(1).unwrap()) {
        Ok(f) => {
            match f {
                Function::Display => run_display(),
                Function::Save => save_image()
            }
        },
        Err(s) => panic!("{}", s),
    }
}

fn run_display() {
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN).unwrap();
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default(); // This array will store the web-cam data

    loop {
        cam.read(&mut frame).unwrap();
        highgui::imshow("window", &frame).unwrap();
        let key = highgui::wait_key(1).unwrap();
        if key == 113 { // quit with q
            break;
        }
    }
    
    highgui::destroy_all_windows()
        .expect("failed to delete all UI elements");
}

fn save_image() {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default(); 
    let params:Vector<i32> = Vector::new();

    cam.read(&mut frame).unwrap();
    imgcodecs::imwrite("test.jpg", &frame, &params)
        .expect("didn't work");
    
    cam.release()
        .expect("Couldn't release video device");
}