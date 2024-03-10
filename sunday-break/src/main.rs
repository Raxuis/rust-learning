use std::io;

trait Table {
    fn new(width: u32, height: u32) -> Self;
    fn print(&self);
}

struct TableImpl {
    width: u32,
    height: u32,
}

impl Table for TableImpl {
    fn new(width: u32, height: u32) -> Self {
        TableImpl { width, height }
    }

    fn print(&self) {
        println!(
            "Table with width: {} and height: {}",
            self.width, self.height
        );
    }
}

fn main() {
    println!("What's the width of your table?");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Error reading input");

    println!("What's the height of your table?");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Error reading input");

    let width: u32 = width.trim().parse().expect("Invalid width");
    let height: u32 = height.trim().parse().expect("Invalid height");

    let table = TableImpl::new(width, height);
    table.print();
}
