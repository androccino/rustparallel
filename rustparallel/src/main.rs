use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    let value = args.nth(1).unwrap();
    let einheit = args.nth(0).unwrap();
    println!("Hello, world!{}, {}",value,einheit);
}
