use std::io;

trait Table {
    fn new(&mut self, width: u32, height: u32) -> Self;
    fn print(&self);
}

struct TableImpl;

impl Table for TableImpl {
    fn new(&mut self, width: u32, height: u32) -> Self {
        TableImpl
    }

    fn print(&self) {
        println!("Table");
    }
}

fn main() {
    println!("What the width or your table?");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Error reading input");
    println!("What the height or your table?");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Error reading input");
    let table = TableImpl::new(
        width.trim().parse().unwrap(),
        height.trim().parse().unwrap(),
    );
    table.print();
}
