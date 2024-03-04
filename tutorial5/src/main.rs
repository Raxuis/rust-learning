fn main() {
    let x: u8 = 9; // 0 <-> 255
    let y: i8 = 10; // -128 <-> 127

    let z = x + y;
    println!("{}", z); // cannot do this because no implementation for `u8 + i8`
}
