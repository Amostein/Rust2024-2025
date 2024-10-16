fn main() {
    let x=99;
    let mut y=x;
    while y>1 {
    println!("{} bottles of beer on the wall,", y);
    println!("{} bottles of beer.", y);
    println!("Take one down, pass it around,");
    y=y-1;
    println!("{} bottles of beer on the wall.", y);
    println!("");
    }

    println!("{} bottle of beer on the wall,", y);
    println!("{} bottle of beer.", y);
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
    println!("");
    
    println!("No bottles of beer on the wall,");
    println!("No bottles of beer.");
    println!("Go to the store, buy some more,");
    println!("{} bottles of beer on the wall.", x);
    
}