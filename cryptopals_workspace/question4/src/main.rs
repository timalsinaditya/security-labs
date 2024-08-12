use std::fs::File;
use std::io::{BufRead, BufReader};
use hex;

fn main() {
    let ascii_chars: Vec<u8> = (97..=122).chain(std::iter::once(32)).collect();

    let path = "src/4.txt";
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let decoded = hex::decode(line.trim()).expect("Failed to decode hex");

        for i in 0..=255 {
            let key = vec![i as u8; decoded.len()];
            let xored = bit_xor(&decoded, &key);
            if text_prob(&xored, &ascii_chars) {
                println!("Key: {:?}, Decrypted: {}", i, String::from_utf8_lossy(&xored));
            }
        }
    }
}

fn bit_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

fn ascii_ratio(a: &[u8], ascii_chars: &[u8]) -> f32 {
    let n_ascii = a.iter().filter(|&&x| ascii_chars.contains(&x)).count();
    n_ascii as f32 / a.len() as f32
}

fn text_prob(a: &[u8], ascii_chars: &[u8]) -> bool {
    ascii_ratio(a, ascii_chars) > 0.75
}
