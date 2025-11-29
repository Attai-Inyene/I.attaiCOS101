use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // ------------------------------
    // 1. Separate datasets (arrays)
    // ------------------------------
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Eteiye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // -------------------------------------------------
    // 2. Merge datasets into a vector of tuples
    // -------------------------------------------------
    let mut merged_data: Vec<(usize, &str, &str, &str)> = Vec::new();

    for i in 0..commissioners.len() {
        merged_data.push((
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i],
        ));
    }

    // -------------------------------------------------
    // 3. Display merged dataset
    // -------------------------------------------------
    println!(
        "{:<5} {:<30} {:<20} {}",
        "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEO-POLITICAL ZONE"
    );
    println!("--------------------------------------------------------------------------");

    for (sn, name, ministry, zone) in &merged_data {
        println!("{:<5} {:<30} {:<20} {}", sn, name, ministry, zone);
    }

    // -------------------------------------------------
    // 4. Save merged dataset to file
    // -------------------------------------------------
    let mut file = File::create("efcc_ministers_merged.csv")?;

    writeln!(
        file,
        "S/N,Name of Commissioner,Ministry,Geopolitical Zone"
    )?;

    for (sn, name, ministry, zone) in &merged_data {
        writeln!(file, "{},{},{},{}", sn, name, ministry, zone)?;
    }

    println!("\nMerged data saved to 'efcc_ministers_merged.csv'!");

    Ok(())
}
