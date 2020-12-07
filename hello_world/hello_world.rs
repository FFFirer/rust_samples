fn main() {
    let num=12;
    println!("Hello world! {} boys", num);
    println!("Hello world! {0}", get_name());
}

fn get_name() -> &'static str {
    return "I'm king of the world";        
}
