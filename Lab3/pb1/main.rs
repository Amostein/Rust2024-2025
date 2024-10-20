fn prime(x: u16) -> bool {
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d <= x/2 {
        if x % d == 0 {
            return false;
        }
        d = d + 2;
    }
    return true;
}
fn next_prime(x: u16) -> Option<u16>
{
    let mut thenext = x.checked_add(1)?;
    while let Some(urm) = thenext.checked_add(0){
        if prime(urm) {
            return Some(urm);
        }
        thenext = thenext.checked_add(1)?;
    }
    None
}

fn main() {
    let mut x = 1;
    while let Some(prime) = next_prime(x) {
        println!("{}", prime);
        x = prime;
    }
}
