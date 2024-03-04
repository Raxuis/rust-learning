fn main() {
    let x: u8 = 255; // 0 <-> 255
    let y: u8 = 1; // 0 <-> 255

    let z = x + y;
    println!("{}", z); // cannot do this because attempt to compute `u8::MAX + 1_u8`, which would overflow
}
