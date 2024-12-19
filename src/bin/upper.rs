extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env::args;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let output: String;
    let mut input: Vec<String> = args().collect();
    input.remove(0);

    if input.len() == 0 {
        output = ctx.get_contents().unwrap().to_uppercase();
    } else {
        output = input.join(" ").to_uppercase();
    }

    println!("{}", output);
    ctx.set_contents(output).unwrap();
}
