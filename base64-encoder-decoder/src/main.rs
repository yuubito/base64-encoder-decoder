mod action;
mod prompt;
use action::{bdecode, bencode};
use prompt::{take_choice, take_input, Choice};

fn main() {
    base64_encoder_decoder();
}
fn base64_encoder_decoder() {
    loop {
        match take_choice(take_input()) {
            Choice::Exit => break,
            Choice::Encode(data) => println!("{}", bencode(data)),
            Choice::Decode(data) => println!("{}", bdecode(data)),
        }
    }
}
