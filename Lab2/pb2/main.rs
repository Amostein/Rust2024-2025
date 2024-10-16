fn add_chars_n(s: &mut String, c: char, x: u32) {
    let letter: String = c.to_string();
    let mut i = 0;
    while i < x {
        s.push_str(&letter);
        i = i + 1;
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n(&mut s, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
}
