use std::io;

fn trapezium() {
    let mut input = String::new();
    println!("Enter length of first base:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f64 = input.trim().parse().expect("Invalid input");
    
    input.clear();
    println!("Enter length of second base:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let h: f64 = input.trim().parse().expect("Invalid input");

    let area = ((a + b) * h) / 2.0;
    println!("Area of the trapezium is = {}", area);
}

fn rhombus() {
    let mut input = String::new();
    println!("Enter first diagonal:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let d1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter second diagonal:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let d2: f64 = input.trim().parse().expect("Invalid input");

    let area = (d1 * d2) / 2.0;
    println!("Area of the rhombus is {}", area);
}

fn parallelogram() {
    let mut input = String::new();
    println!("Enter base length:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid input");

    let area = base * height;
    println!("Area of the parallelogram is {}", area);
}

fn cube() {
    let mut input = String::new();
    println!("Enter edge length:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f64 = input.trim().parse().expect("Invalid input");

    let area = 6.0 * a.powi(2);
    println!("Surface area of the cube is {}", area);
}

fn cylinder() {
    let mut input = String::new();
    println!("Enter cylinder base radius:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let r: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter cylinder height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let h: f64 = input.trim().parse().expect("Invalid input");

    let volume = std::f64::consts::PI * r.powi(2) * h;
    println!("Volume of the cylinder is {}", volume);
}

fn main() {
    println!("\n------ ATTAI'S GEOMETRY SOLVER -----------");
    let subjects_arr = ["Trapezium", "Rhombus", "Parallelogram", "Cube", "Cylinder"];
    println!("Pick one of the following shapes:");
    for shape in &subjects_arr {
        println!("- {}", shape);
    }

    let mut shape = String::new();
    io::stdin().read_line(&mut shape).expect("Failed to read line");
    let shape = shape.trim().to_lowercase();

    match shape.as_str() {
        "trapezium" => trapezium(),
        "rhombus" => rhombus(),
        "parallelogram" => parallelogram(),
        "cube" => cube(),
        "cylinder" => cylinder(),
        _ => println!("Invalid shape input, try again."),
    }
}
