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
