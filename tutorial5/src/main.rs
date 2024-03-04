fn main() {
    let x: u8 = 255; // 0 <-> 255
    let y: u8 = 10; // 0 <-> 255

    let z = x / y;
    println!("{}", z); // Gives 25 but the result is 25.5
}
