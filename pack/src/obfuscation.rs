use brotli::CompressorReader;
use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};
use std::{
    fs::File,
    io::{self, Read, Seek, Write},
};

pub(crate) fn encrypt_and_compress_large_file(
    source_file_path: &str,
    dest_file: &mut File,
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> io::Result<()> {
    let mut cipher = ChaCha20::new(key.as_ref().into(), nonce.as_ref().into());

    const BUFFER_LEN: usize = 8196;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;

    let size_of_source_file = source_file.stream_len()?;

    let mut size_of_compressed_file: u64 = 0;

    let mut reader = CompressorReader::new(source_file, 0, 11, 24);

    while let Ok(count) = reader.read(&mut buffer) && count > 0 {
        cipher.apply_keystream(&mut buffer);
        dest_file.write(&buffer[..count])?;
        size_of_compressed_file += count as u64;
    }
    
    colour::cyan_ln!(
        "
            Compressed the target crate by {:.2}%

            Size then: {size_of_source_file}, size now: {size_of_compressed_file}
        ",
        (1.0 - size_of_compressed_file as f32 / size_of_source_file as f32) * 100.0,
    );

    Ok(())
}
