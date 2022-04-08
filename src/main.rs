mod decoded;
mod utf8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    args.into_iter().for_each(|word| {
        let chars = word.chars();
        println!("Word: {:?}", chars);
        for c in chars {
            let decoded_char = utf8::char_decoder::decode(c);
            println!("{:?}", decoded_char);
        }
        println!();
    });
}
