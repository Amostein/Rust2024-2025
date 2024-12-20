fn coprime(x: u32, y: u32) {
    if x == 0 || y == 0 {
        return;
    }
    if x % 2 == 0 && y % 2 == 0 {
        return;
    }
    let mut d = 3;
    while d <= x && d <= y {
        if x % d == 0 && y % d == 0 {
            return;
        }
        d = d + 2;
    }
    println!("{} and {} are coprime", x, y);
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    while x < 101 {
        while y < 101 {
            coprime(x, y);
            y = y + 1;
        }
        y = 0;
        x = x + 1;
    }
}
