use std::{io::Read, mem};

pub fn load_bin<S: Read>(src: S, dest: &mut [u8]) -> extern "system" fn() {

    
    unsafe {
        mem::transmute::<*const u8, extern "system" fn()>(0u8 as *const u8)
    }
}

// #![feature(ptr_metadata)]

// use pelite::{
//     pe64::{Pe, PeFile},
//     Error as PeError,
// };

// use core::ptr::{from_raw_parts, Thin};

// use windows_sys::{
//     core::PCSTR,
//     Win32::System::{
//         LibraryLoader::{GetModuleHandleA, LoadLibraryA},
//         Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
//     },
// };

// pub unsafe fn load_pe(pe_data: &[u8]) -> Result<fn(), PeError> {
//     let pe_file = PeFile::from_bytes(pe_data)?;
//     let pe_nt_headers = pe_file.nt_headers();

//     let image_base = GetModuleHandleA(null()) as *mut _;

//     let mut old_protection_flags = PAGE_PROTECTION_FLAGS::default();

//     VirtualProtect(
//         image_base,
//         pe_nt_headers.OptionalHeader.SizeOfHeaders as usize,
//         PAGE_READWRITE,
//         &mut old_protection_flags,
//     );

//     memcpy(
//         image_base as *mut u8,
//         &pe_data,
//         pe_nt_headers.OptionalHeader.SizeOfHeaders as usize,
//     );

//     for section in pe_file.section_headers() {
//         let dst = image_base.add(section.VirtualAddress as usize);

//         if section.SizeOfRawData > 0 {
//             VirtualProtect(
//                 dst,
//                 section.SizeOfRawData as usize,
//                 PAGE_READWRITE,
//                 &mut old_protection_flags,
//             );
//             memcpy(
//                 dst as *mut u8,
//                 std::slice::from_raw_parts(
//                     section.PointerToRawData as *const u8,
//                     section.SizeOfRawData as usize,
//                 ),
//                 section.SizeOfRawData as usize,
//             )
//         } else {
//             VirtualProtect(
//                 dst,
//                 section.VirtualSize as usize,
//                 PAGE_READWRITE,
//                 &mut old_protection_flags,
//             );
//             memset(dst as *mut u8, 0, section.VirtualSize as usize)
//         }
//     }

//     println!("copied sections");

//     for import_descriptor in pe_file.imports()? {
//         let import_module =
//             LoadLibraryA(import_descriptor.dll_name().unwrap().to_string().as_ptr() as PCSTR);
//     }

//     Ok(std::mem::transmute::<*const (), fn()>(
//         image_base.add(pe_nt_headers.OptionalHeader.AddressOfEntryPoint as usize) as *const (),
//     ))
// }

// unsafe fn memcpy(dst: *mut u8, src: &[u8], size: usize) {
//     let dst = std::slice::from_raw_parts_mut(dst, size);
//     for i in 0..size {
//         dst[i] = src[i]
//     }
// }

// unsafe fn memset(dst: *mut u8, byte: u8, size: usize) {
//     let dst = std::slice::from_raw_parts_mut(dst, size);

//     for i in 0..size {
//         dst[i] = byte;
//     }
// }

// fn decrypted_pe_data(data: &[u8]) -> Vec<u8> {
//     todo!()
// }

// pub const fn null<T: ?Sized + Thin>() -> *const T {
//     from_raw_parts(0 as *const (), ())
// }