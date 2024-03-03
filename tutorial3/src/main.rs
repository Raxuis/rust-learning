fn main() {
    let mut tup: (i32, bool, char) = (1, true, 's');
    println!("First tuple : {:?}", tup);
    tup.0 = 33;
    println!("Edited tuple : {:?}", tup);
}
