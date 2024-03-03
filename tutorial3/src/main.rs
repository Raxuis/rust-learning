fn main() {
    let tup: (i32, bool, char) = (1, true, 's');
    let _tup2: (i32, f64, char) = (1, 19.32, 's');
    println!("{:?}", tup);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}
