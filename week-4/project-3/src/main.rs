//rust rogrma to calculate price of goods

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

//to find the number of goods purchased
 println!("Enter item code: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let code:bool = input1.trim().parse().expect("Not a valid number");

 println!("Enter quantity purchased: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let q:f32 = input2.trim().parse().expect("Not a valid number");
     
    let L = 550_000.0;
    let M = 120_000.0;
    let K = 15_000.0;
    let H = 25_000.0;

    if code == L
    {
        let code = 550_000;
    }
    else if code == M
    {
    let code = 120_000; 
    }
    else if code == K
    {
    let code = 15_000;    
    }
    else if code == H
    {
    let code = 15_000;
    }


    let sum = code * q;

    if sum >= 500_000.0 && sum <=0.0
    {
    let sum = sum * 0.7;
    }
    else {
        sum = sum * 1.0;
    }

    println!("your total is ${}", sum);
}