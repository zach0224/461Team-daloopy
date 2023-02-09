pub struct Package {
    total_score: i32,
    bus_factor: i32,
    responsiveness: i32,
    license: bool,
   //url: String // change later to URL type
}

trait InputPackage {
    fn print_output(self);
}

impl InputPackage for Package {
    
    fn print_output(self) { 
        //println!("URL {}", self.url);
        println!("Total score {}", self.total_score);
        println!("Bus Factor {}", self.bus_factor);
        println!("Responsiveness {}", self.responsiveness);
        println!("License Compatibility {}", self.license);
    }
}

pub struct URL {
    url: String
}

trait InputURL {
    //fn find_URL(self) -> String // nmp to github url

    fn get_url_string(self); // return the url string
    //fn setUrlString(self) -> // void???
}

impl InputURL for URL {
    //fn find_URL(&self) {
        //find url and set the url variable to that string
    //}
    fn get_url_string(self) {
        // stuff
        println!("url {}", self.url);

    }
}

/*impl Package {
    //use like constructors
    pub fn new() -> Package {
        Package {
            BusFactor;

        }
    }
}*/

/*impl URL{
    pub fn new() -> URL{
        URL{
            url;
        }
    }
}*/