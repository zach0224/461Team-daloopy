use std::fmt;

pub struct Package {
    pub total_score: i32,
    pub bus_factor: i32,
    pub responsiveness: i32,
    pub license: bool,
    pub url: URL // change later to URL type
}


impl Package {
    
    fn print_output(self) { 
        println!("URL {}", self.url);
        println!("Total score {}", self.total_score);
        println!("Bus Factor {}", self.bus_factor);
        println!("Responsiveness {}", self.responsiveness);
        println!("License Compatibility {}", self.license);
    }
}



pub struct URL {
    pub url: String,
    pub git_repo_url: String
}

impl URL {

    pub fn new(url: String) -> URL{
        URL {url: url, git_repo_url: url}
    }

    pub fn get_url_string(&self) -> String{
        println!("url {}", self);
        self.url
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}
