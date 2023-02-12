use std::fmt;
use std::path::Path;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json;
use package::Package;

#[derive(Deserialize)]
struct MetricJSON {
    pub license:  f32,
    pub issues: IssuesJSON,
    pub total_commits: i32,
}

#[derive(Deserialize)]
struct IssuesJSON {
    pub open: i32,
    pub closed: i32,
    pub total: i32,
}

pub fn calc_metrics(package: Package, filepath: String){
    let path = Path::new(filepath);
    let read_result = fs::read_to_string(path); // Open the path in read-only mode, returns `io::Result<File>`
    let _string match file_Result {
        Ok(_string) => {
            let json: MetricJSON = serde_json::read_from_str(_string)?;
            package.bus_factor = bus_factor(json);
            package.responsiveness = responsiveness(json);
            package.correctness = json.correctness;
            package.ramp = ramp_up_time(json);
            package.license = json.license;
        }
        Err(err) =>{
            panic!("Problem opening the file: {:?}", err)
        }
    }
}

pub fn bus_factor(json: MetricJSON) -> f32 {
    let total_commits : i32 = json.total_commits;
    let top_contributor_commits : i32 = json.top_commits;
    let ratio : f32 = top_contributor_commits as f32 / total_commits as f32;
    (1.0 - ratio) * 10.0
}

pub fn responsiveness(json: MetricJSON) -> f32 {
    let open: i32 = json.issues.open + 50;
    let total: i32 = json.issues.total + 100;
    open as f32 / total as f32
}

pub fn ramp_up_time(json: MetricJSON) -> f32 {
    let wiki: f32 = json.has_wiki;
    let discussions: f32 = json.has_discussions;
    let pages: f32 = json.has_pages;
    let readme: f32 = json.has_readme;
    0.25 * wiki + 0.25 * discussions + 0.25 * pages + 0.25 * readme
}



