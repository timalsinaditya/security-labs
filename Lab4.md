# LAB-4 Cryptopals Set 1

## [Press here to return to HOME](index.md)

## Cryptopals Set 1 

### Question 1

```bash
#!/bin/bash

if [ $# -ne 1 ]; then 
    echo "Usage: $0 <hex_string>"
    exit 1
fi

echo "$1" | xxd -r -p | base64
```

Here is the output of the above script:

```bash 
./question1.sh 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d

SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
```

### Question 2 
The C implementation of the second code is shown below:
```c
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

void xor_bytes(const uint8_t *b1, const uint8_t *b2, uint8_t *result, size_t length) {
    for (size_t i = 0; i < length; i++) {
        result[i] = b1[i] ^ b2[i];
    }
}

void print_hex(const uint8_t *data, size_t length) {
    for (size_t i = 0; i < length; i++) {
        printf("%02x", data[i]);
    }
    printf("\n");
}

int main() {
    const char *hex1 = "1c0111001f010100061a024b53535009181c";
    const char *hex2 = "686974207468652062756c6c277320657965";
    
    size_t length = strlen(hex1) / 2;
    uint8_t b1[length], b2[length], result[length];
    
    for (size_t i = 0; i < length; i++) {
        sscanf(hex1 + 2 * i, "%2hhx", &b1[i]);
        sscanf(hex2 + 2 * i, "%2hhx", &b2[i]);
    }
    
    xor_bytes(b1, b2, result, length);
    
    print_hex(result, length);
    
    return 0;
}
```

Here is the output of the above code:

```bash
gcc -o main question2.c && ./main
746865206b696420646f6e277420706c6179
```

### Question 3
The rust implementation of the third question is shown below.
```rust
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
```

Here is the output of the above code: 
```bash 
cargo build -p question3
cargo run -p question3 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

Decrypted: Cooking MC's like a pound of bacon
```

### Question 4

The rust implementation is show below:
```rust
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
```

The output looks like this
```bash
#please ensure you are in question4 directory
cargo build -p question4
cargo run -p question4

Key: 53, Decrypted: Now that the party is jumping
```

### Question 5
```rust 
use hex;

fn main() {
    let input_str = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let keystream: Vec<u8> = key.iter().cycle().take(input_str.len()).cloned().collect();

    let xored = bit_xor(input_str, &keystream);
    println!("{}", hex::encode(xored));
}

fn bit_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}
```

The output looks like this:
```bash
cargo build -p question5
cargo run -p question5

0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
```

### Question 6

The rust implementation is shown below:

```rust
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
```

Here is the output of the above code: 
```bash 
#please ensure you are in question6 directory
cargo build -p question6
cargo run -p question6
```

### Quesiton 7 

The python implementation of this question is shown below:

```python 
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
from base64 import b64decode

backend = default_backend()

def decrypt_aes(cyphertext, key):
    cipher = Cipher(algorithms.AES(key), modes.ECB(), backend=backend)
    decryptor = cipher.decryptor()
    
    decrypted_msg =  decryptor.update(cyphertext) + decryptor.finalize()
    return decrypted_msg

key = b"YELLOW SUBMARINE"
with open("7.txt") as in_file:
    input_str = in_file.read().strip()
    decrypted_msg = decrypt_aes(b64decode(input_str), key)
    print(decrypted_msg.decode("utf-8"))
```

Here is the output:
```bash
python3 question7.py 
```

### Question 8 
The implementation is shown below: 
```python
from binascii import unhexlify

key_len = 16

def is_prob_ecb(text):
    if len(text) % key_len != 0:
        return False
    n_blocks = len(text) // key_len
    block_size = key_len
    blocks = [text[i*block_size:(i+1)*block_size] for i in range(n_blocks)]

    if(len(set(blocks)) < len(blocks)):
        return True
    else:
        return False

with open("8.txt") as in_file:
    print("Probable ECB encoded text:")
    for line in in_file:
        if(is_prob_ecb(unhexlify(line.strip()))):
            print(unhexlify(line.strip()))
```