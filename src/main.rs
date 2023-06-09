use core::panic;
use std::fs::read;
const SIX_BIT_MASK: u32 = 0x3F;
const TABLE: [char; 64]= ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c',
'd','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];

fn main() {
    let input_name = "input.txt";
    let content = read(input_name)
    .unwrap_or_else(|e| panic!("{}", e.to_string()));

    let mut output = String::new();
    for chunk in content.chunks(3){
        output.push_str(encode_bytes(chunk).as_str())
    }

    println!("{output}");
}

pub fn encode_bytes(bytes: &[u8]) -> String {

    let mut bit_pool: u32 = 0;
    let mut encoded = String::new();

    if bytes.len() > 3 || bytes.len() == 0{
        panic!("Length must be betweeen 3 and 1");
    }

    for (index, byte) in bytes.iter().enumerate() {
        bit_pool |= (*byte as u32) << (24 - 8 * index);
    }

    let sextets = bytes.len() + 1; // 2,3 or 4

    for index in 1..=sextets {
        let v = (bit_pool >> (32 -  6 * index)) & SIX_BIT_MASK;
        encoded.push(TABLE[v as usize]);
    }

    while encoded.len() < 4 {
        encoded.push('=');
    }

    encoded
}
