use std::io;

fn main() {
    println!("Type something :");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("What you typed : {}", input);
}
