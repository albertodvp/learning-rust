fn main() {
    let mut x = 5; // immutable
    println!("{x}");
    x = 6;
    println!("{x}");
    let mut small: u8 = 255;
    small = small+1;
    println!("{small}");
}
