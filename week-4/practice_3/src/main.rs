//rust program to calculate the area of a
//triangle for a given base and height

 use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("Not a avalid string");
    let base:f32 = input1.trim().parse().expect("Not a valid nummber");


    println!("Enter height: ");
    io::stdin().read_line(&mut input2).expect("Not a avalid string");
    let height:f32 = input2.trim().parse().expect("Not a valid nummber");

    if base > 0.0{
        let area:f32 =(base * height) / 2.0;
        println!("Area of a triangle: {}", area);
    }
}