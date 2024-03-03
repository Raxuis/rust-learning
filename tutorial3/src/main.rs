fn main() {
    let mut tup: (i32, bool, char) = (1, true, 's');
    println!("Tuple first value : {}", tup.0);
    println!("First tuple : {:?}\n", tup);
    tup.0 = 33;
    println!("Tuple first value : {}", tup.0);
    println!("Edited tuple : {:?}\n", tup);

    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Array first value : {}", arr[0]);
    println!("First array : {:?}", arr);
}
