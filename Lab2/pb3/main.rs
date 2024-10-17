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
    let mut ok = 0;
    if lung %3!=0{
        s.push_str(&numar[step..step+lung%3]);
        step = step + lung % 3;
        ok = 1;
    }

    while step<lung {
        if ok == 1 {
            s.push_str("_");
        }
        s.push_str(&numar[step..step+3]);
        step = step + 3;
        if step < lung {
        s.push_str("_");
        }
    }
}

fn add_float(s: &mut String, flt: f32) {
    s.push_str(&flt.to_string());
}

fn main() {
    let mut s = String::from("");
    add_space(&mut s, 38);
    add_str(&mut s, (&"I").to_string());
    add_space(&mut s, 1);
    add_str(&mut s, (&"💚").to_string());
    println!("{}", s);
    s.clear();
    
    add_space(&mut s, 38);
    add_str(&mut s, (&"RUST.").to_string());
    println!("{}", s);
    s.clear();

    add_str(&mut s, (&"Most").to_string());
    add_space(&mut s, (&("downloaded").len()+2).try_into().unwrap());

    add_str(&mut s, (&"crate").to_string());
    add_space(&mut s, (&("has").len()+2).try_into().unwrap());

    let number=306437968;
    add_integer(&mut s, number);
    add_space(&mut s, (&("downloads").len()+2).try_into().unwrap());

    add_str(&mut s, (&"and").to_string());
    add_space(&mut s, (&("the").len()+2).try_into().unwrap());

    add_str(&mut s, (&"lastest").to_string());
    add_space(&mut s, (&("version").len()+2).try_into().unwrap());

    add_str(&mut s, (&"is").to_string());
    println!("{}", s);
    s.clear();

    add_space(&mut s, (&("Most").len()+1).try_into().unwrap());

    add_str(&mut s, (&"downloaded").to_string());
    add_space(&mut s, (&("crate").len()+2).try_into().unwrap());

    add_str(&mut s, (&"has").to_string());
    add_space(&mut s, ((&"306_437_968").len()+2).try_into().unwrap());

    add_str(&mut s, (&"downloads").to_string());
    add_space(&mut s, (&("and").len()+2).try_into().unwrap());

    add_str(&mut s, (&"the").to_string());
    add_space(&mut s, (&("lastest").len()+2).try_into().unwrap());

    add_str(&mut s, (&"version").to_string());
    add_space(&mut s, (&("is").len()+2).try_into().unwrap());

    add_float(&mut s, 2.038);
    add_str(&mut s, (&".").to_string());
    println!("{}", s);
}
