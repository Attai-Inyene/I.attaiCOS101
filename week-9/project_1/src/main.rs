use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Categories from the table
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Write each category into its own file
    write_category("lager.txt", &lager)?;
    write_category("stout.txt", &stout)?;
    write_category("non_alcoholic.txt", &non_alcoholic)?;

    println!("Files created successfully!");

    Ok(())
}

fn write_category(filename: &str, items: &Vec<&str>) -> Result<()> {
    let mut file = File::create(filename)?;

    for item in items {
        writeln!(file, "{}", item)?;
    }

    Ok(())
}
