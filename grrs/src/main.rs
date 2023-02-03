use std::env;
use std::path::Path;

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
        _ => handleFile(task),
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

fn handleFile(urlfile){
    println!("inside handle URL");

    let path = Path::new(urlfile);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    
    // parse file


}
