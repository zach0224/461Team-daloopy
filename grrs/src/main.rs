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


pub fn main(){

    let args: Vec<String> = env::args().collect(); //returns an iterator

    let task = &args[1]; //stores what instruction will be run
    println!("File to run {}", task);

    let path = Path::new(task.as_str());
    let file_result = File::open(path); // Open the path in read-only mode, returns `io::Result<File>`

    // error handling
    let _file = match file_result  {
        Ok(_file) => {
            let reader = BufReader::new(_file);
            let mut heap = BinaryHeap::<Package>::new();
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); // Ignore errors.
                println!("{}. {}", index + 1, line);

                // initialize object
                // might not be needed
                let mut package = Package::new(line);
                let python_code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/api.py"));

                println!("Constructed Package");
                println!("Running Python:");
                let result = Python::with_gil(|py| -> Result<String, PyErr> {
                    let code = PyModule::from_code(py, python_code, "", "").unwrap();
                    let temp: String = code.getattr("getData")?.call1((package.url.get_owner_repo(),))?.extract()?;
                    Ok(temp)
                });
                let json = result.unwrap();
                package.calc_metrics(&json);
                heap.push(package);
            }
            while !heap.is_empty() {
                let temp = heap.pop().unwrap();
                temp.print_output();
                let json = PackageJSON::new(&temp);
                let json_string = serde_json::to_string(&json).unwrap();
                println!("{}", json_string);
            }
        }
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
