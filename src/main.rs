#[warn(unused)]
use std::{fs::File};

fn main() {
    let inputName = "input.txt";
    let _input = File::open(inputName).unwrap_or_else(|_| panic!("File {inputName} not found!"));

}
