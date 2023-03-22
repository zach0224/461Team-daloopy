use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod package;
use package::Package;
use package::PackageJSON;
use pyo3::prelude::*;
use std::collections::BinaryHeap;
use serde_json;
use log::LevelFilter;
use log::{info, debug};

/// Wrapper function resposnbile for obtaining the filepath of the ***URL_FILE*** and passes it to
/// [handle_url_file()] to process
pub fn main(){

    let args: Vec<String> = env::args().collect(); //returns an iterator

    let task = &args[1]; //stores what instruction will be run
    let log_path = &args[2]; //stores what instruction will be run
    let temp = &args[3]; //stores what instruction will be run
    let log_level: i32 = temp.parse::<i32>().unwrap();
    handle_url_file(task.to_string(), log_path.to_string(), log_level);
}

pub fn handle_url_file(url_file_path: String, log_path: String, log_level: i32){
    // init logging functions
    let level: LevelFilter;
    if log_level == 2 {
        level = LevelFilter::Debug;
    } else if log_level == 1 {
        level = LevelFilter::Info;
    } else {
        level = LevelFilter:: Off;
    }

    // The File::create(&log_path) function attempts to create a file at the path 
    // specified by the log_path variable, and returns a Result value that represents 
    // the success or failure of the operation.
    let result = File::create(&log_path);
    match result {
        Ok(..) => {
            // the simple_logging crate is used to log messages to the file at the specified level. 
            simple_logging::log_to_file(log_path, level).unwrap();
        }
        Err(_e) => {
            simple_logging::log_to_stderr(level);
        }
    }

    // start url processing

    info!("URL File to run {}", url_file_path);

    // This code opens a file located at the path specified by the url_file_path variable, 
    // using the File::open() function. The Path::new() function is used to create a new Path 
    // object representing the file path, which is passed as an argument to the File::open() function.
    let path = Path::new(url_file_path.as_str());
    let file_result = File::open(path); // Open the path in read-only mode, returns `io::Result<File>`
    // error handling
    let _file = match file_result  {
        Ok(_file) => {
            debug!("File handled Properly");
            let reader = BufReader::new(_file);
            let mut heap = BinaryHeap::<Package>::new();
            // for every line in URL_file
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); // Ignore errors.
                info!("{}. {}", index + 1, line);

                // initialize object
                // Does the URL Processing (Regexing happens here)
                let mut package = Package::new(line); // look at package.rs (impl package))
                // (Regexing stops here)
                
                // env!("CARGO_MANIFEST_DIR") is an environment variable provided by Cargo, the Rust package manager, 
                // that contains the path to the root directory of the current package.

                // The path to api.py is constructed by concatenating the value of the CARGO_MANIFEST_DIR environment variable, 
                // which contains the path to the root directory of the current package, with the string "/api.py". 
                // The resulting string is then passed to the include_str! macro, which includes the contents of the file as a 
                // string literal.
                let python_code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/api.py"));
                // Logs the Github url returned from either the npmjs or github url file 
                info!("Constructed Package {}", package.url.get_url());

                // Does the URL Processing
                // Execute python code to obtain metric data
                debug!("Running Python");
                let result = Python::with_gil(|py| -> Result<String, PyErr> {
                    let code = PyModule::from_code(py, python_code, "", "").unwrap();
                    let temp: String = code.getattr("getData")?.call1((package.url.get_owner_repo(),))?.extract()?;
                    Ok(temp)
                });
                debug!("Python returned successfully");
                let json = result.unwrap(); // result is of json type
                package.calc_metrics(&json);
                // push each package onto the heap (sorted my metric scores)
                heap.push(package);
            }
            while !heap.is_empty() {
                // pop out the highest element from heap
                let temp = heap.pop().unwrap();
                temp.debug_output();
                // Convert to a new PackageJSON (look at package.rs)
                let json = PackageJSON::new(&temp);
                // Serialize it to regular text
                let json_string = serde_json::to_string(&json).unwrap();
                println!("{}", json_string);
            }
        }
        Err(err) => info!("Problem opening the file: {:?}", err),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let _args = vec![
            "program_name".to_owned(),
            "task".to_owned(),
            "log_path".to_owned(),
            "3".to_owned(),
        ];

        let result = handle_url_file("task".to_owned(), "log_path".to_owned(), 3);
        assert_eq!(result, ());
    }

    #[test]
    fn test_handle_url_file() {
        let url_file_path = "URLs.txt".to_owned();
        let log_path = "log.txt".to_owned();
        let log_level = 2;

        let result = handle_url_file(url_file_path, log_path, log_level);

        // Perform your assertions here.
        // For example:
        assert_eq!(result, ());
    }
}

