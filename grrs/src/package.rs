use std::fmt;

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
        URL {url: url.clone(), git_repo_url: url.clone()}
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
