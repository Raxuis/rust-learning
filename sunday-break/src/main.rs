use std::io;

fn main() {
    println!("What's the height of your table?");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Can't read it!");
    println!("{}", height);
}
