use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod package;
use package::Package;
use package::URL;
use octocrab::Octocrab;

fn main() {
    let args: Vec<String> = env::args().collect(); //returns an iterator

    let task = &args[1]; //stores what instruction will be run
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
                println!("{}. {}", index + 1, line);

                // initialize object
                // might not be needed
                let obj = Package {
                    total_score: -1,
                    bus_factor: -1,
                    responsiveness: -1,
                    license: false,
                    url: URL { url: line }, // send in URL
                };

                // convert url npm to github (before or after using API?)
                // struct & auth -> tmw office hours Anonya & William 
                // overloading -> Will

                // npm to github url -> Jason 
                // classes updating (package.rs clean up)
                // one giant struct with each calculation function (empty)
                // npm github api in rust (super close)
                // 

                // graphql in 
                // 

                // tmw pieceing files together and calc scores

                // get content -> Dalilah
                    // get content -> use APIs octocrab (Dalilah) get into 
                
                


                // call functions
                // calc_response(); 
                // calc_license();
                // calc_bus_factor();
                // calc_rampup();
                // calc_correctness(); 
                // calc_total_score();


            }
        }
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
}



