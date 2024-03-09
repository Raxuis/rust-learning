use std::io;

fn main() {
    println!("What do you offer to me?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "cookie" => println!("Miam a delicious {}.", input),
        "fruit" => println!("A {} is not that bad for my health!", input),
        _ => println!("Why would I eat {} when I can eat cookies.", input),
    }
}
