use std::env;
use hex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <hex_string>", args[0]);
        std::process::exit(1);
    }

    let hex_string = &args[1];
    let input_str = hex::decode(hex_string).expect("Invalid hex string");

    for i in 0..=255 {
        let key = vec![i as u8; input_str.len()];
        let xored = bit_xor(&input_str, &key);
        if text_prob(&xored) {
            println!("Key: {:?}, Decrypted: {}", i, String::from_utf8_lossy(&xored));
        }
    }
}

fn bit_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

fn ascii_ratio(a: &[u8]) -> f32 {
    let ascii_chars: Vec<u8> = (97..=122).chain(std::iter::once(32)).collect();
    let n_ascii = a.iter().filter(|&&x| ascii_chars.contains(&x)).count();
    n_ascii as f32 / a.len() as f32
}

fn text_prob(a: &[u8]) -> bool {
    ascii_ratio(a) > 0.75
}
