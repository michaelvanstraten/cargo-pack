use winapi::um::{libloaderapi::GetModuleHandleA};

// use crate::constants::SIZE_OF_UNCOMPRESSED_BINARY;

// pub(crate) fn allocate_data(aslr: bool) {
//     if !aslr {
//         unsafe {
//             let image_base = GetModuleHandleA(std::ptr::null()) as *mut u8;
//             let mut target_buffer = std::slice::from_raw_parts_mut(image_base, SIZE_OF_UNCOMPRESSED_BINARY);
//         };
//     } else {
//         let mut target_crate_buffer: Vec<u8> = Vec::with_capacity(SIZE_OF_UNCOMPRESSED_BINARY);
//     }
// }

// pub(crate) fn load_pe_into_memory(src: &[u8], dst: &mut [u8]) {
//     let reader = Decompressor::new(compressed_data, 0);
//     let image_dos_header = IMAGE_DOS_HEADER::re
// }