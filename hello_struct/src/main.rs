// use std::fmt;


fn main() {
    println!("Hello, world!");
    play_struct();
}

// define a struct
#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32        
}

impl Site {
    fn double_found(&self) -> u32{
        self.found * 2
    }

    fn create(domain: String, name: String, nation: String, found: u32) -> Site{
        Site{
            domain,
            name,
            nation,
            found
        }
    }
}

/*
impl fmt::Display for Site{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "domain:{}, name:{}, nation:{}, found:{}", self.domain, self.name, self.nation, self.found);
    }
}
*/

#[derive(Debug)]
struct Color(u8, u8, u8);

/*
impl fmt::Display for Color{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2);
    }
}*/

fn play_struct() {
    // new a struct
    let website = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };
    
    println!("website is {:?}", website);
        
    // new another struct
    let baidu = Site {
        domain: String::from("www.baidu.com"),
        ..website
    };
    
    println!("baidu is {:?}", baidu);

    let soho_name = String::from("soho");
    let soho = Site {
        name: soho_name,
        ..baidu
    };
    
    println!("soho is {:?}", soho);
    println!("double soho is {}", soho.double_found());

    println!("tencet is {:?}", Site::create(String::from("www.tencert.com"), String::from("tencert"), String::from("China"), 2014));

    // tuple struct
    let black = Color(0,0,0);

    println!("black is {:?}", black);
}

