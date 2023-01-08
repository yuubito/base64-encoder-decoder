// This is the prompt module.
use std::io;

pub enum Choice {
    Encode(String),
    Decode(String),
    Exit,
}
pub fn take_input() -> String {
    println!("\nenter the data:");
    let mut input_data: String = String::new();
    io::stdin().read_line(&mut input_data).expect("err");
    let data: String = input_data.trim().into();
    data
}
pub fn take_choice(data: String) -> Choice {
    println!("\n1- for encoding");
    println!("2- for decoding\n");
    let mut input_number: String = String::new();
    io::stdin().read_line(&mut input_number).expect("err");
    let number: u8 = input_number.trim().parse().expect("err");

    match number {
        0 => Choice::Exit,
        1 => Choice::Encode(data),
        2 => Choice::Decode(data),
        _ => take_choice(data),
    }
}
