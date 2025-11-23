use std::io;

// Define a structure to hold the data for each Public Service (APS) level.
#[derive(Debug, Clone)]
struct StaffLevel {
    designation: &'static str,
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

impl StaffLevel {
    // A method to check if a given title matches any of the professions at this level.
    fn matches_title(&self, profession: &str, title: &str) -> bool {
        match profession.to_lowercase().as_str() {
            "office administrator" => self.office_admin.to_lowercase() == title.to_lowercase(),
            "academic" => self.academic.to_lowercase() == title.to_lowercase(),
            "lawyer" => self.lawyer.to_lowercase() == title.to_lowercase(),
            "teacher" => self.teacher.to_lowercase() == title.to_lowercase(),
            _ => false, // Invalid profession
        }
    }
}

// Function to initialize the dataset from the provided table.
fn initialize_staff_levels() -> Vec<StaffLevel> {
    vec![
        StaffLevel {
            designation: "APS 1-2",
            office_admin: "Intern",
            academic: "-",
            lawyer: "Paralegal",
            teacher: "Placement",
        },
        StaffLevel {
            designation: "APS 3-5",
            office_admin: "Administrator",
            academic: "Research Assistant",
            lawyer: "Junior Associate",
            teacher: "Classroom Teacher",
        },
        StaffLevel {
            designation: "APS 5-8",
            office_admin: "Senior Administrator",
            academic: "PhD Candidate",
            lawyer: "Associate",
            teacher: "Snr Teacher",
        },
        StaffLevel {
            designation: "EL1 8-10",
            office_admin: "Office Manager",
            academic: "Post-Doc Researcher",
            lawyer: "Senior Associate 1-2",
            teacher: "Leading Teacher",
        },
        StaffLevel {
            designation: "EL2 10-13",
            office_admin: "Director",
            academic: "Senior Lecturer",
            lawyer: "Senior Associate 3-4",
            teacher: "Deputy Principal",
        },
        StaffLevel {
            designation: "SES",
            office_admin: "CEO",
            academic: "Dean",
            lawyer: "Partner",
            teacher: "Principal",
        },
    ]
}

// Function to validate the staff level and return the corresponding APS designation.
fn get_staff_aps_level(
    staff_levels: &Vec<StaffLevel>,
    profession: &str,
    title: &str,
) -> Option<&'static str> {
    staff_levels
        .iter()
        .find(|level| level.matches_title(profession, title))
        .map(|level| level.designation)
}

// --- NEW FUNCTION TO HANDLE USER INPUT ---
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    // Trim whitespace and newline characters from the input
    input.trim().to_string() 
}

// ----------------------------------------

fn main() {
    let staff_levels = initialize_staff_levels();
    println!("--- 🏛️ Public Service APS Level Checker ---");
    println!("Available Professions: Office Administrator, Academic, Lawyer, Teacher.");
    println!("------------------------------------------");

    // Get user input for Profession
    let profession = read_input("➡️ Enter the Staff Member's Profession:");

    // Get user input for Job Title
    let title = read_input("➡️ Enter the Staff Member's Job Title:");
    
    println!("\n--- Calculating APS Level ---");
    println!("Staff Profession: **{}**", profession);
    println!("Staff Job Title: **{}**", title);

    // Perform the validation
    let result = get_staff_aps_level(&staff_levels, &profession, &title);

    // Output the result
    match result {
        Some(aps) => println!("\n🎉 RESULT: The staff member holds position **{}**.", aps),
        None => println!("\n❌ RESULT: **No corresponding APS level found** for that Title and Profession combination."),
    }

    println!("-----------------------------");
}