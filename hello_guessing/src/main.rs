use std::io;

fn main() {
    println!("Guess a number!");
    println!("Input your guess number");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("can't read line");

    println!("Your guess number is {}", num);

}
