use std::fmt;
use regex::Regex;
use lazy_static::lazy_static;
use reqwest;

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
        println!("URL from Package:       {}", self.url.get_url_string());
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
        let git_url = URL::get_git_url(&url);
        URL {url: url.clone(), git_repo_url: git_url}
    }

    fn get_git_url(url: &String) -> String{
        lazy_static! {
            static ref GIT_RE:Regex = Regex::new(r#"https://github\.com/.+"#).unwrap();
            static ref NPM_RE:Regex = Regex::new(r#"https://www\.npmjs\.com/package/(.+)"#).unwrap();
        }
        if GIT_RE.is_match(url) {
            println!("{} is a github URL!", url);
            url.clone()
        } else {
            let cap = NPM_RE.captures(url).unwrap();
            println!("Package Name: {:?}", &cap[1]);
            let npm_url = format!("https://registry.npmjs.org/{}", &cap[1]);
            println!("NPM URL: {}", npm_url);
            let body = reqwest::blocking::get(npm_url);
            println!("body = {:?}", body);
            url.clone()
        }
    }
    pub fn get_url_string(&self) -> String{
        println!("URL from URL: {}", self);
        self.url.clone()
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}
