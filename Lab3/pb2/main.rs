
fn chheck_add(x: u32, y: u32) -> u32
{
    let (suma, overflow) = x.overflowing_add(y);
    if overflow {
        panic!("Add overflow");
    }
    suma
}

fn chheck_mul(x: u32, y: u32) -> u32
{
    let (prod, overflow) = x.overflowing_mul(y);
    if overflow {
        panic!("Multiplication overflow");
    }
    prod
}

fn main() {
    let x = 2200000000;
    let y = 2200000000;
    // let y = 1200000000 -> mult overflow
    //u32 ~ 429....
    let sum = chheck_add(x,y);
    println!("Add: {}", sum);
    let prod = chheck_mul(x,y);
    println!("Mult: {}", prod);
}
