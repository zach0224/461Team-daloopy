use std::env;

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
        _ => println!("Error not a valid input"),
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
