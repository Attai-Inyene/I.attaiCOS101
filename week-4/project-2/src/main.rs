//rust program to calculate comound interest

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

 println!("Enter money deposited: ");
    io::stdin().read_line(&mut input1).expect("Not a avalid string");
    let p:f32 = input1.trim().parse().expect("Not a valid number");

 println!("Enter interest rate: ");
    io::stdin().read_line(&mut input2).expect("Not a avalid string");
    let r:f32 = input2.trim().parse().expect("Not a valid number");

 println!("Enter number of years: ");
    io::stdin().read_line(&mut input3).expect("Not a avalid string");
    let t:i32 = input3.trim().parse().expect("Not a valid number");

    let a:f32 = (1.0 + r) / 100.0;
    let b:f32 = a.powi(t);
    let c:f32 = b * p;

    let ci:f32 = c - p;

    println!("your compond interest is {}", ci);
    println!("your total amount is {}", c);  
}
