use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // -------------------------------
    // 1. Create a vector of tuples
    // -------------------------------
    // (Student Name, Matric Number, Department, Level)
    let students: Vec<(&str, &str, &str, u32)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10101001", "Economics", 100),
        ("Shania Bolade", "CSC10328288", "Computer", 200),
        ("Adekunle Gold", "EEE10202002", "Electrical", 200),
        ("Blande Edemoh", "MEE10200041", "Mechanical", 100),
    ];

    // -------------------------------
    // 2. Display the details
    // -------------------------------
    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {}", 
        "Student Name", "Matric Number", "Department", "Level");
    println!("-----------------------------------------------------------------");

    for (name, matric, dept, level) in &students {
        println!("{:<20} {:<15} {:<15} {}", name, matric, dept, level);
    }

    // -------------------------------
    // 3. Save to file (CSV format)
    // -------------------------------
    let mut file = File::create("students.csv")?;

    // Write header
    writeln!(
        file,
        "Student Name,Matric Number,Department,Level"
    )?;

    // Write each student
    for (name, matric, dept, level) in &students {
        writeln!(file, "{},{},{},{}", name, matric, dept, level)?;
    }

    println!("\nData successfully saved to 'students.csv'!");

    Ok(())
}
