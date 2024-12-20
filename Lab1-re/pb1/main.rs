fn prime(x: u32) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d = d + 2;
    }
    return true;
}

fn main() {
    let mut x = 0;
    while x < 101 {
        if prime(x) {
            println!("{} is prime.", x);
        } else {
            println!("{} is not prime", x);
        }
        x = x + 1;
    }
}
