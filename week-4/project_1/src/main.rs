//rust program to find the rootss of an equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter coefficient a");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not  valid number");

    println!("Enter coefficient b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not  valid number");
    
    println!("Enter coefficient c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not  valid number");

    let discriminant:f32 = b.powi(2) - 4.0 * a * c;
    let s:f32 = discriminant.sqrt();

    if discriminant > 0.0 {
        println!("root nature: two distinct and real");
    }
    if discriminant == 0.0 {
        println!("root nature: two equal real roots");
    }
    if discriminant < 0.0 {
        println!("two complex roots");
    }   
    let x:f32 = ((-1.0 * b) + s) / (2.0 * a);
    let y:f32 = ((-1.0 * b) - s) / (2.0 * a);

    println!("\nEquation: {}x^2 + {}x {} = 0",a, b, c );
    println!("roots are {} and {}", x, y ); 
    }

