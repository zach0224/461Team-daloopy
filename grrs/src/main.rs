use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

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
    println!("inside handle URL");

    let path = Path::new(urlfile);

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    
    // HAVENT TESTED OR INCORPORATED:
    // parse file
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    if let Ok(lines) = read_lines(&file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}