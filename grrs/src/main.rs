use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod package;
use package::Package;
use package::URL;


pub extern "C" fn main() -> i32{
    let args: Vec<String> = env::args().collect(); //returns an iterator

    let task = &args[1]; //stores what instruction will be run
    println!("Task to run {}", task);

    match task.as_str(){
        "install" => install(),
        "build" => build(),
        "test" => test(),
        _ => handle_file(task.as_str()),
    }
    return 1;
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
                println!("{}. {}", index + 1, line);

                // initialize object
                // might not be needed
                let obj = Package {
                    total_score: -1.0,
                    bus_factor: -1.0,
                    responsiveness: -1.0,
                    license: -1.0,
                    correctness: -1.0,
                    ramp: -1.0,
                    url: URL::new(line), // send in URL
                };

                obj.print_output();
                obj.calc_metrics(urlfile);
            }
        }
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
}



