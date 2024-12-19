use std::env::args;

fn main() {
    let mut input: Vec<String> = args().collect();
    input.remove(0);

    println!("{}", input.join(" ").to_uppercase());
}
