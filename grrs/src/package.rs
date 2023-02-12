use std::fmt;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;

use reqwest;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct NpmJSON {
    repository:  HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct MetricJSON {
    pub license:  f32,

    pub open_issues: i32,
    pub total_issues: i32,

    pub has_wiki: bool,
    pub has_discussions: bool,
    pub has_readme: bool,
    pub has_pages: bool,

    pub total_commits: i32,
    pub top_commits: i32,

    pub correctness: f32,
}

pub struct Package {
    pub total_score: f32,
    pub bus_factor: f32,
    pub responsiveness: f32,
    pub correctness: f32,
    pub ramp: f32,
    pub license: f32,
    pub url: URL // change later to URL type
}


impl Package {
    
    pub fn print_output(&self) { 
        println!("");
        println!("Package URL:            {}", self.url.get_url());
        println!("Owner/Repo:             {}", self.url.get_owner_repo());
        println!("Total score:            {}", self.total_score);
        println!("Bus Factor:             {}", self.bus_factor);
        println!("Responsiveness:         {}", self.responsiveness);
        println!("Correctness:            {}", self.correctness);
        println!("Ramp Up Time:           {}", self.ramp);
        println!("License Compatibility:  {}", self.license);
        println!("");
    }

    pub fn calc_metrics(mut self, filepath: &str){
        let path = Path::new(&filepath);
        let read_result = fs::read_to_string(path).expect("Unable to read file");
        let json: MetricJSON = serde_json::from_str(&read_result).expect("Unable to parse JSON");
        self.bus_factor = bus_factor(&json);
        self.responsiveness = responsiveness(&json);
        self.correctness = json.correctness;
        self.ramp = ramp_up_time(&json);
        self.license = json.license;
    }
    
}


pub struct URL {
    pub url: String,
    pub owner_repo: String
}


impl URL {

    pub fn new(url: String) -> URL{
        let owner_repo = URL::determine_owner_repo(&url);
        URL {url: url.clone(), owner_repo: owner_repo}
    }

    fn determine_owner_repo(url: &String) -> String{
        lazy_static! {
            static ref GIT_RE:Regex = Regex::new(r#".+github\.com/(.+)"#).unwrap();
            static ref NPM_RE:Regex = Regex::new(r#"https://www\.npmjs\.com/package/(.+)"#).unwrap();
            static ref GIT_NPM_RE:Regex = Regex::new(r#".+github\.com/(.+).git"#).unwrap();
        }
        if GIT_RE.is_match(url) {
            println!("{} is a github URL!", url);
            let owner_repo = GIT_RE.captures(url).unwrap();
            println!("{} is the owner repo!", &owner_repo[1]);
            (&owner_repo[1]).to_string()
        } else {
            println!("{} is NOT a github URL!", url);
            let cap = NPM_RE.captures(url).unwrap();
            let npm_url = format!("https://registry.npmjs.org/{}", &cap[1]);
            let response = reqwest::blocking::get(npm_url).unwrap();
            let json = response.json::<NpmJSON>().unwrap();
            let git_url_from_npm = json.repository.get("url").unwrap();
            let owner_repo = GIT_NPM_RE.captures(&git_url_from_npm).unwrap();
            println!("{} is the owner repo!", &owner_repo[1]);
            (&owner_repo[1]).to_string()
        }
    }
    pub fn get_url(&self) -> String{
        self.url.clone()
    }
    pub fn get_owner_repo(&self) -> String{
        self.owner_repo.clone()
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

pub fn bus_factor(json: &MetricJSON) -> f32 {
    let total_commits : i32 = json.total_commits;
    let top_contributor_commits : i32 = json.top_commits;
    let ratio : f32 = top_contributor_commits as f32 / total_commits as f32;
    (1.0 - ratio) * 10.0
}

pub fn responsiveness(json: &MetricJSON) -> f32 {
    let open: i32 = json.open_issues + 50;
    let total: i32 = json.total_issues + 100;
    open as f32 / total as f32
}

pub fn ramp_up_time(json: &MetricJSON) -> f32 {
    let wiki:        f32 = (json.has_wiki as i32)        as f32;
    let discussions: f32 = (json.has_discussions as i32) as f32;
    let pages:       f32 = (json.has_pages as i32)       as f32;
    let readme:      f32 = (json.has_readme as i32)      as f32;
    0.25 * wiki + 0.25 * discussions + 0.25 * pages + 0.25 * readme
}



