use std::io;

// Define an enum for the possible types of roots, making the function signature clean.
#[derive(Debug)]
enum QuadraticRoots {
    TwoReal(f64, f64),
    OneReal(f64),
    TwoComplex(f64, f64), // (real part, imaginary part)
    LinearEquation(f64),
    NoSolution,
}

/// Helper function to read a coefficient (a, b, or c) from the user, handling input errors.
fn read_coefficient(name: &str) -> f64 {
    loop {
        println!("Enter coefficient {}:", name);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Attempt to parse the trimmed input string as a floating-point number (f64)
                match input.trim().parse::<f64>() {
                    Ok(val) => return val,
                    Err(_) => println!("Error: Please enter a valid number."),
                }
            }
            Err(e) => println!("Error reading input: {}", e),
        }
    }
}

/// Calculates the roots of a quadratic equation (ax^2 + bx + c = 0).
fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticRoots {
    // Check for non-quadratic cases first
    if a == 0.0 {
        if b == 0.0 {
            // Case: c = 0 (No solution if c != 0, infinite if c = 0. We'll categorize as NoSolution if c != 0)
            return QuadraticRoots::NoSolution; 
        } else {
            // Case: bx + c = 0 (Linear equation)
            let root = -c / b;
            return QuadraticRoots::LinearEquation(root);
        }
    }

    // Calculate the discriminant: delta = b^2 - 4ac
    let discriminant = b.powi(2) - 4.0 * a * c;
    let two_a = 2.0 * a;

    if discriminant > 0.0 {
        // Two distinct real roots
        let sqrt_delta = discriminant.sqrt();
        let root1 = (-b + sqrt_delta) / two_a;
        let root2 = (-b - sqrt_delta) / two_a;
        QuadraticRoots::TwoReal(root1, root2)
    } else if discriminant == 0.0 {
        // One real repeated root
        let root = -b / two_a;
        QuadraticRoots::OneReal(root)
    } else {
        // Two complex roots: x = (-b / 2a) +/- (sqrt(|delta|) / 2a) * i
        let abs_discriminant = discriminant.abs();
        let sqrt_abs_delta = abs_discriminant.sqrt();

        let real_part = -b / two_a;
        let imaginary_part = sqrt_abs_delta / two_a;

        QuadraticRoots::TwoComplex(real_part, imaginary_part)
    }
}

fn main() {
    println!("--- Quadratic Equation Solver (ax^2 + bx + c = 0) ---");

    // Read coefficients from user input
    let a = read_coefficient("a");
    let b = read_coefficient("b");
    let c = read_coefficient("c");

    // Display the equation for context
    println!("\nSolving equation: ({})x^2 + ({})x + ({}) = 0", a, b, c);

    let roots = solve_quadratic(a, b, c);

    // Use pattern matching to print the result cleanly
    match roots {
        QuadraticRoots::TwoReal(r1, r2) => {
            println!("Root nature: Two distinct real roots.");
            println!("Root 1: {:.4}", r1);
            println!("Root 2: {:.4}", r2);
        }
        QuadraticRoots::OneReal(r) => {
            println!("Root nature: One real repeated root.");
            println!("Root: {:.4}", r);
        }
        QuadraticRoots::TwoComplex(real, imag) => {
            println!("Root nature: Two complex conjugate roots.");
            println!("Root 1: {:.4} + {:.4}i", real, imag);
            println!("Root 2: {:.4} - {:.4}i", real, imag);
        }
        QuadraticRoots::LinearEquation(r) => {
            println!("This is a linear equation (a=0).");
            println!("Root: {:.4}", r);
        }
        QuadraticRoots::NoSolution => {
            println!("This equation has no solution (or infinite solutions if c=0).");
        }
    }
}