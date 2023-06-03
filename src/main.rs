use std::{fs::{read}};

fn main() {
    let input_name = "input.txt";
    let content = read(input_name).unwrap_or_else(|e| panic!("{}", e.to_string()));

    let mut output = String::new();
    for three_bytes in content.chunks(3){
        output.push_str(byte3_to_char4(three_bytes).as_str());
    }

    println!("{output}");
}


pub fn byte3_to_char4(bytes: &[u8]) -> String {
    const SIX_BIT_MASK: u32 = 0x3F;
    const TABLE: [char; 64]= ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c',
    'd','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];

    let mut bit_pool: u32 = 0;

    for (index, byte) in bytes.iter().rev().enumerate() {
        bit_pool += (*byte as u32) << (8 * index) ;
    }

    let mut values = Vec::new();

    // 1 -> 0, 1 .rev()
    // 2 -> 0, 1, 2 .rev()
    // 3 -> 0, 1, 2, 3 .rev()
    for index in (0..=bytes.len()).rev() {
        let v = ((bit_pool >> (index * 6)) & SIX_BIT_MASK) as usize;
        values.push(v);
    } 

    let mut encoded = String::new();

    for value in values {
        encoded.push(TABLE[value]);
    }

    while encoded.len() < 4 {
        encoded.push('=');
    }

    encoded
}
