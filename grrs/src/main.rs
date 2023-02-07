use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect(); //returns an iterator

    let run = &args[1]; //store the "run" string
    let task = &args[2]; //stores what instruction will be run

    println!("command inserted {}", run);
    println!("Task to run {}", task);

    match task.as_str(){
        "install" => install(),
        "build" => build(),
        "test" => test(),
        _ => handle_file(task.as_str()),
    }
}
fn install(){
    println!("In install");
}
fn build(){
    println!("In build");
}
fn test(){
    println!("In test");
}

fn handle_file(urlfile:&str){
    //println!("inside handle URL");

    let path = Path::new(urlfile);
    let file_result = File::open(path); // Open the path in read-only mode, returns `io::Result<File>`

    // error handling
    let _file = match file_result  {
        Ok(_file) => {
            let reader = BufReader::new(_file); 
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); // Ignore errors.
                // Show the line and its number.
                //println!("inside for loop");
                println!("{}. {}", index + 1, line);
            }
        }
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
}
