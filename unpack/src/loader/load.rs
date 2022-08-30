use std::{io::Read, mem};

pub fn load_bin<S: Read>(src: S, dest: &mut [u8]) -> extern "system" fn() {

    
    unsafe {
        mem::transmute::<*const u8, extern "system" fn()>(0u8 as *const u8)
    }
}