fn add_space(s: &mut String, n: u32) {
    let mut i=0;
    let spc: &str = " ";
    while i < n {
        i+=1;
        s.push_str(&spc);
}
}

fn add_str(s: &mut String, after: String) {
    s.push_str(&after);
}

fn add_integer(s: &mut String, x: u32) {
    let numar = x.to_string();
    let lung = numar.len();
    let mut step = 0;
    let mut copyleng = lung;
    if copyleng%3!=0{
        s.push_str(&numar[step..step+1]);
        copyleng = copyleng - 1;
        step = step+1;
        if copyleng%3!=0{
            s.push_str(&numar[step..step+1]);
            copyleng = copyleng -1;
            step = step+1;
        }
    }

    while step<lung {
        s.push_str(&numar[step..step+3]);
        s.push_str("_");
        step = step + 3;
    }
}

fn add_float(s: &mut String, flt: f32) {
    s.push_str(&flt.to_string());
}

fn main() {
    let mut s = String::from("");
    add_str(&mut s, (&"Oh dear").to_string());
    add_space(&mut s, 2);
    let number=12223;
    add_integer(&mut s, number);
    add_space(&mut s, 3);
    add_float(&mut s, 124.78);
}
