fn main() {
    // Compound conditions
    // && : and
    // || : or
    // ! : not
    let cond = (2 as f32) < 3.4;
    let cond2 = false && cond;
    println!("{}", cond);
    println!("{}", cond2);
}
