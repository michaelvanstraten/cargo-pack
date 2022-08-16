use object::{read::pe::PeFile64, LittleEndian};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("CARGO_FEATURE_NO_ASLR").is_ok() {
        setup_no_alsr()?;
    }
    Ok(())
}

fn setup_no_alsr() -> Result<(), Box<dyn Error>> {
    let previous_compilation = std::env::var("PREVIOUS_COMPILATION")?;
    let input_image_base: u32 = std::env::var("INPUT_IMAGE_BASE")?.parse()?;
    let shift_rva: i32 = std::env::var("SHIFT_RVA")?.parse()?;

    let bin_data = fs::read(previous_compilation)?;
    let pe_file = PeFile64::parse(&*bin_data)?;

    println!(
        "cargo:rustc-link-arg=-Wl,--image-base={:x}",
        input_image_base
    );

    for section in pe_file.section_table().iter() {
        println!(
            "cargo:rustc-link-arg=-Wl,--section-start={}={:x}",
            String::from_utf8_lossy(section.raw_name()),
            (input_image_base + section.virtual_address.get(LittleEndian)) as i32 + shift_rva
        );
    }

    Ok(())
}
