use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("host.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() >= 2 {
            println!("{} => {}", columns[1], columns[0]);
        }
    }

    Ok(())
}
