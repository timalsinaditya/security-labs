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