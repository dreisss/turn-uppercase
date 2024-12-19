extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env::args;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut input: Vec<String> = args().collect();
    input.remove(0);

    let output = input.join(" ").to_uppercase();
    println!("{}", output);
    ctx.set_contents(output).unwrap();
}
