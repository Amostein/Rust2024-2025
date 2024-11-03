use std::fs::File;
use std::io::{self, BufRead};
#[derive(Debug)]
struct Student {
    nume: String,
    telefon: String,
    varsta: u32,
}

fn main() {
    let mut studenti: Vec<Student> = Vec::new();
    let file = File::open("studenti.txt").unwrap();
    let reader = io::BufReader::new(file);

    for rand in reader.lines() {
        let rand = rand.unwrap();
        let parts: Vec<&str> = rand.split(',').collect();
        let nume = parts[0].to_string();
        let telefon = parts[1].to_string();
        let varsta = parts[2].parse::<u32>().unwrap();
        studenti.push(Student {
            nume,
            telefon,
            varsta,
        });
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
        youngest.nume, youngest.telefon, youngest.varsta
    );
    println!(
        "Oldest: {} {} {}",
        oldest.nume, oldest.telefon, oldest.varsta
    );
}
