use std::io;

fn main() {
    run_plus_v2();
}

fn plus_number(x: i32, y: i32) -> i32{
    return x+y;  
}

/*
fn run_plus() {
    let mut input_string = String::new();
    println!("Input Number A:");
    io::stdin().read_line(&mut input_string).expect("can't read line: Number A");

    let num_str_a = input_string.trim();
    println!("Number A is {}", num_str_a);
    let num_a = num_str_a.parse::<i32>().unwrap();

    input_string = String::new();
    println!("Input Number B:");
    io::stdin().read_line(&mut input_string).expect("can't read line: Number B");

    let num_str_b = input_string.trim();
    println!("Number B is {}", num_str_b);
    let num_b = num_str_b.parse::<i32>().unwrap();

    let num_c = plus_number(num_a, num_b);
    println!("A + B = {}", num_c);
}
*/

fn run_plus_v2(){
    let mut num_str_a = String::new();
    println!("Input Number A:");
    io::stdin().read_line(&mut num_str_a).expect("can't read line: Number A");
    // println!("Your Input Number A is >{}<", num_str_a);
    let num_str_a = num_str_a.trim().parse::<i32>().unwrap();

    let mut num_str_b = String::new();
    println!("Input Number B:");
    io::stdin().read_line(&mut num_str_b).expect("can't read line: Number B");
    // println!("Your Input Number B is >{}<", num_str_b);
    let num_str_b = num_str_b.trim().parse::<i32>().unwrap();

    let num_c = plus_number(num_str_a, num_str_b);
    println!("A + B = {}", num_c);
}
