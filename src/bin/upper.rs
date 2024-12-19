extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env::args;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut input: Vec<String> = args().collect();
    input.remove(0);

    let output: String = if input.is_empty() {
        ctx.get_contents().unwrap().to_uppercase()
    } else {
        input.join(" ").to_uppercase()
    };

    println!("{}", output);
    ctx.set_contents(output).unwrap();
}
