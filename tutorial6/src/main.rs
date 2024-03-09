fn main() {
    // Compound conditions
    // && : and
    // || : or
    // ! : not
    let cond = (2 as f32) < 3.4;
    let cond2 = false && cond;
    let cond3 = false || cond;
    let cond4 = !cond;
    println!("{}", cond);
    println!("{}", cond2);
    println!("{}", cond3);
    println!("{}", cond4);
}
