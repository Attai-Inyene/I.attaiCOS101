//rust program to calculate comound interest

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

 println!("Enter money deposited: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f32 = input1.trim().parse().expect("Not a valid number");

 println!("Enter interest rate: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let r:f32 = input2.trim().parse().expect("Not a valid number");

 println!("Enter number of years: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let t:f32 = input3.trim().parse().expect("Not a valid number");

    let a:f32 = (1.0 + r) / 100.0;
    let b:f32 = a.powf(t as f32);
    let c:f32 = b * p;

    let ci = p + c;

    println!("your compond interest is {}", c);
    println!("your total amount is {}", ci);  
}
