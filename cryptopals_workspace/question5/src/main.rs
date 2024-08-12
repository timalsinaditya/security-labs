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