//rust rogrma to calculate price of goods

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

//to find the number of goods purchased
 println!("Enter item code: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let code:f32 = input1.trim().parse().expect("Not a valid number");

 println!("Enter quantity purchased: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let q:f32 = input2.trim().parse().expect("Not a valid number");
     
    let l = 550_000.0;
    let m = 120_000.0;
    let k = 15_000.0;
    let h = 25_000.0;

    if code == l
    {
        let code = 550_000;
    }
    else if code == m
    {
    let code = 120_000; 
    }
    else if code == k
    {
    let code = 15_000;    
    }
    else if code == h
    {
    let code = 15_000;
    }


    let mut sum = code * q;

    if sum >= 500_000.0 && sum <=0.0
    {
    let sum = sum * 0.7;
    }
    else {
        sum = sum * 1.0;
    }

    println!("your total is ${}", sum);
}