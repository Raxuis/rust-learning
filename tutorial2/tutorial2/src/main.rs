fn main() {
    let x = 4;
    println!("x = {}", x);

    {
        let x = x - 2;
        println!("x = {}", x);
    }

    let x = 5;
    println!("x = {}", x);
}
