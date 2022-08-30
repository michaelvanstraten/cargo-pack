use std::io::{self, Cursor, Read};

use super::constants::ENCRYPTED_BINARY_LENGTH;
use brotli_decompressor::Decompressor;
use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};

pub fn decrypt_data(
    encrypted_data: &'static mut [u8; ENCRYPTED_BINARY_LENGTH],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> &'static [u8; ENCRYPTED_BINARY_LENGTH] {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(encrypted_data);
    &*encrypted_data
}

pub fn decompress_data(compress_data: &[u8], dest: &mut Vec<u8>) -> io::Result<usize> {
    let mut reader = Cursor::new(compress_data);
    
    let mut decompressor = Decompressor::new(reader, 0);

    decompressor.read_to_end(dest)
}
