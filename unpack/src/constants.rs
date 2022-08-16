use konst::{primitive::parse_u64, result::unwrap_ctx};
use std::{env, time::Duration};

// pub(super) const SIZE_OF_UNCOMPRESSED_BINARY: usize =
//     unwrap_ctx!(parse_usize(env!("SIZE_OF_UNCOMPRESSED_BINARY")));

pub(super) const ENCRYPTED_BINARY_LENGTH: usize =
    include_bytes!(env!("ENCRYPTED_BINARY_PATH")).len();

pub(super) static mut ENCRYPTED_BINARY: [u8; ENCRYPTED_BINARY_LENGTH] =
    *include_bytes!(env!("ENCRYPTED_BINARY_PATH"));

pub(super) const ENCRYPTION_KEY: &[u8; 32] = include_bytes!(env!("ENCRYPTION_KEY_PATH"));

pub(super) const ENCRYPTION_NONCE: &[u8; 12] = include_bytes!(env!("ENCRYPTION_NONCE_PATH"));

pub(super) const TIMEOUT: Duration = Duration::from_secs(unwrap_ctx!(parse_u64(env!("TIMEOUT"))));
