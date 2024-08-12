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