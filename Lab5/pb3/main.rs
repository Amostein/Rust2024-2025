use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]

struct Student {
    nume: String,
    phone: String,
    varsta: u32,
}

fn main() {
    let mut studenti: Vec<Student> = Vec::new();
    for file_name in &["constantin.json", "mihai.json", "elena.json", "diana.json"] {
        let content = fs::read_to_string(file_name).unwrap();
        let stud: Student = serde_json::from_str(&content).unwrap();
        studenti.push(stud);
    }

    let mut oldest = &studenti[0];
    let mut youngest = &studenti[0];
    for candidat in &studenti {
        if candidat.varsta > oldest.varsta {
            oldest = candidat;
        }
        if candidat.varsta < youngest.varsta {
            youngest = candidat;
        }
    }
    println!(
        "Youngest: {} {} {}",
        youngest.nume, youngest.phone, youngest.varsta
    );
    println!("Oldest: {} {} {}", oldest.nume, oldest.phone, oldest.varsta);
}
