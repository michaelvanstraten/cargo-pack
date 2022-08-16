use brotli::Decompressor;
use chacha20::{ChaCha20, cipher::{KeyIvInit, StreamCipher}};
use super::constants::ENCRYPTED_BINARY_LENGTH;

pub fn decrypt_data(encrypted_data: &'static mut [u8; ENCRYPTED_BINARY_LENGTH], key: &[u8; 32], nonce: &[u8; 12]) -> &'static [u8; ENCRYPTED_BINARY_LENGTH] {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(encrypted_data);
    &*encrypted_data
}

pub fn decompress_data(compressed_data: &[u8]) {
    let reader = Decompressor::new(compressed_data, 0);
}
