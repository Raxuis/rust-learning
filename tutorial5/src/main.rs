fn main() {
    let x: i64 = 9; // 0 <-> 255
    let y: i8 = 10; // -128 <-> 127

    let z = x + y;
    println!("{}", z); // cannot do this because no implementation for `i64 + i8`
}
