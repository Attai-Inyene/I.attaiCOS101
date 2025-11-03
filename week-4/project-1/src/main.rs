//rust pogram to determine student scores

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a avalid string");

    println!("Enter your  first score: ");
    io::stdin().read_line(&mut input2).expect("Not a avalid string");
    let score2:f32 = input2.trim().parse().expect("Not a valid number");

 println!("Enter your  second score: ");
    io::stdin().read_line(&mut input3).expect("Not a avalid string");
    let score3:f32 = input3.trim().parse().expect("Not a valid number");

 println!("Enter your third score: ");
    io::stdin().read_line(&mut input4).expect("Not a avalid string");
    let score4:f32 = input4.trim().parse().expect("Not a valid number");

    let sum:f32 = score4 + score2 + score3;
    let average:f32 = sum / 3.0;

    println!("name: {}", input1);
    
    if average > 100.0
    {
        println!("invalid score");
    }
    if average >= 70.0 && average <= 100.0{
        println!("score: A");
    }
    else if average >=60.0 && average <=69.0
    {
        println!("score: B");
    }
    else if average >= 50.0 && average <= 59.0
    {
        println!("Score: C");
    }
    else if average >= 45.0 && average <= 49.0
    {
        println!("Score: D");
    }
    else 
    {
        println!("Score: F");
    }

    if average < 0.0
    {
        println!("Invalid score", );
    }
}

