use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");
    let value_response = value_check(input.trim().parse().unwrap());
    if value_response {
        println!("Your input is an integer");
    } else {
        println!("Your input is not an integer");
    }
}
fn value_check(value: i64) -> bool {
    if value < 0 {
        println!("{} is negative", value);
    } else if value > 0 {
        println!("{} is positive", value);
    } else {
        println!("{} is zero", value);
        return false;
    }
    return true;
}
