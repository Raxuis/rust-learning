use std::io;

fn main() {
    println!("What's the height of your table?");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Can't read it!");
    println!("What's the width of your table?");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Can't read it!");
    println!(
        "Your table's dimensions : {}x{}",
        height.trim(),
        width.trim()
    );
}
