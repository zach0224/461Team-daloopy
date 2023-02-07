pub struct Package {
    totalScore: i32,
    BusFactor: i32,
    Responsiveness: i32,
    License: 0b,
    url: String,

}

impl Package {
    //use like constructors
    pub fn new() -> Package {
        Package {

        }
    }
}

trait inputPackage {
    fn find_URL(self) -> String
    fn print_output(self) -> String
}

impl inputPackage for Package {

    //fn new ()???????? 

    fn find_URL(&self) {
        //find url and set the url variable to that string
    }

    fn print_output(&self) {
        println!("URL {}", self.url);
        println!("Total score {}", self.totalScore);
        println!("Bus Factor {}", self.BusFactor);
        println!("Responsiveness {}", self.Responsiveness);
        println!("License Compatibility {}", self.License);
    }
}
