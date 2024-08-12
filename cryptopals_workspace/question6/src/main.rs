use base64;
use std::fs::read_to_string;

const ASCII_CHARS: [u8; 27] = [32, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122];
const MAX_KEY_SIZE: usize = 30;
const MIN_KEY_SIZE: usize = 2;

fn main() {
    let contents = read_to_string("src/6.txt").expect("Failed to read file");
    let cleaned_contents: String = contents.chars().filter(|c| !c.is_whitespace()).collect();
    let input_str = base64::decode(cleaned_contents).expect("Failed to decode base64");

    let key = find_key(&input_str);
    let keystream: Vec<u8> = key.iter().cycle().take(input_str.len()).cloned().collect();
    let decoded_str = bit_xor(&input_str, &keystream);
    println!("{}", String::from_utf8_lossy(&decoded_str));
}

fn bit_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

fn hamming(a: &[u8], b: &[u8]) -> u32 {
    bit_xor(a, b).iter().map(|&byte| byte.count_ones()).sum()
}

fn calc_key_size(input_str: &[u8]) -> usize {
    (MIN_KEY_SIZE..=MAX_KEY_SIZE)
        .map(|size| {
            let n = input_str.len() / (size * 2) - 1;
            let score: f64 = (0..n)
                .map(|j| {
                    let a = &input_str[j*size*2..j*size*2 + size];
                    let b = &input_str[j*size*2 + size..j*size*2 + size*2];
                    hamming(a, b) as f64
                })
                .sum::<f64>() / (size as f64);
            (size, score)
        })
        .min_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
        .unwrap().0
}

fn ascii_ratio(byte_seq: &[u8]) -> f64 {
    byte_seq.iter().filter(|&&byte| ASCII_CHARS.contains(&byte)).count() as f64 / byte_seq.len() as f64
}

fn text_prob(byte_seq: &[u8]) -> bool {
    ascii_ratio(byte_seq) > 0.75
}

fn solve_single_xor(ciphertext: &[u8]) -> u8 {
    (0..=255u8)
        .max_by(|&a, &b| {
            let score_a = ascii_ratio(&bit_xor(ciphertext, &vec![a; ciphertext.len()]));
            let score_b = ascii_ratio(&bit_xor(ciphertext, &vec![b; ciphertext.len()]));
            score_a.partial_cmp(&score_b).unwrap()
        })
        .unwrap()
}

fn find_key(ciphertext: &[u8]) -> Vec<u8> {
    let size = calc_key_size(ciphertext);
    let n_blocks = ciphertext.len() / size;
    (0..size)
        .map(|j| {
            let block: Vec<u8> = (0..n_blocks).map(|i| ciphertext[i*size + j]).collect();
            solve_single_xor(&block)
        })
        .collect()
}