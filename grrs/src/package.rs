use std::fmt;
use regex::Regex;
use lazy_static::lazy_static;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize)]
struct NpmJson {
    repository:  HashMap<String, String>,
}

pub struct Package {
    pub total_score: i32,
    pub bus_factor: i32,
    pub responsiveness: i32,
    pub correctness: i32,
    pub ramp: i32,
    pub license: bool,
    pub url: URL // change later to URL type
}


impl Package {
    
    pub fn print_output(self) { 
        println!("URL from Package:       {}", self.url.get_git_url());
        println!("Total score:            {}", self.total_score);
        println!("Bus Factor:             {}", self.bus_factor);
        println!("Responsiveness:         {}", self.responsiveness);
        println!("Correctness:            {}", self.correctness);
        println!("Ramp Up Time:           {}", self.ramp);
        println!("License Compatibility:  {}", self.license);
        println!("");
    }
}


pub struct URL {
    pub url: String,
    pub git_repo_url: String
}


impl URL {

    pub fn new(url: String) -> URL{
        let git_url = URL::determine_git_url(&url);
        URL {url: url.clone(), git_repo_url: git_url}
    }

    fn determine_git_url(url: &String) -> String{
        lazy_static! {
            static ref GIT_RE:Regex = Regex::new(r#".+github\.com/(.+)"#).unwrap();
            static ref NPM_RE:Regex = Regex::new(r#"https://www\.npmjs\.com/package/(.+)"#).unwrap();
        }
        if GIT_RE.is_match(url) {
            println!("{} is a github URL!", url);
            url.clone()
        } else {
            println!("{} is NOT a github URL!", url);
            let cap = NPM_RE.captures(url).unwrap();
            let npm_url = format!("https://registry.npmjs.org/{}", &cap[1]);
            let response = reqwest::blocking::get(npm_url).unwrap();
            let json = response.json::<NpmJson>().unwrap();
            let git_url_from_npm = json.repository.get("url").unwrap();
            let temp = GIT_RE.captures(&git_url_from_npm).unwrap();
            let git_url = format!("https://github.com/{}", &temp[1]);
            git_url.clone()
        }
    }
    pub fn get_git_url(&self) -> String{
        self.git_repo_url.clone()
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}
