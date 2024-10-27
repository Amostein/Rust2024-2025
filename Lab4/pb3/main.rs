use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let abbreviations = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna")];
    let input_path = "input.txt";
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    if let Ok(_) = reader.read_line(&mut line) {
        let mut result = Vec::new();
        for word in line.split_whitespace() {
            let mut replaced = false;
            for &(abbr, full_form) in &abbreviations {
                if word == abbr {
                    result.push(full_form);
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                result.push(word);
            }
        }
        let output = result.join(" ");
        println!("{}", output);
    } else {
        eprintln!("Failed to read line");
    }
    Ok(())
}
