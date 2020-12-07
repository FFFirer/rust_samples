fn main() {
    play_enum();
}

#[derive(Debug)]
enum Book {
    Papery{index: u32}, 
    Electronic{url: String}
}

fn play_enum(){
    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("http://....")};

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("Electronic book {}", url);        
        }
    }
    
    // let name = "abc";    
    let name = "abca";
    match name {
        "abc" => println!("Yes"),
        _ => println!("No")
    }
    
    // let opt = Option::Some("Hello");
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => println!("{}", something),
        Option::None => println!("opt is nothing")
    }



}
