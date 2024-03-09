fn main() {
    // Compound conditions
    // && : and
    // || : or
    // ! : not
    let cond = (2 as f32) < 3.4;
    let cond2 = true && cond;
    println!("{}", cond);
    println!("{}", cond2);
}
