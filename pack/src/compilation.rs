use std::{
    io,
    process::{Command, Stdio},
};

use cargo_metadata::Artifact;

use crate::utils;

pub(crate) fn compile_input_target(
    target: &Option<String>,
    cargo_args: Vec<String>,
) -> io::Result<Vec<Artifact>> {
    let mut cmd = cargo_build();

    cmd.args(cargo_args);

    if let Some(target) = target {
        cmd.arg("--target").arg(target);
    }

    Ok(utils::parse_artifacts(&cmd.output()?.stdout))
}

fn cargo_build() -> Command {
    let mut cmd = Command::new("cargo");

    cmd.arg("build")
        .arg("--message-format")
        .arg("json-diagnostic-rendered-ansi")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    cmd
}

fn compile_unpacker_base_command(
    encryption_key_filename: &String,
    nonce_filename: &String,
    compressed_input_target_filename: &String,
    target: &Option<String>,
    timeout: u64,
) -> Command {
    let mut cmd = cargo_build();

    cmd
        .arg("--release")
        .arg("--manifest-path")
        .arg(concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/unpack/Cargo.toml"
        ));

    if let Some(target) = target {
        cmd.arg("--target").arg(target);
    }

    cmd.env("ENCRYPTED_BINARY_PATH", compressed_input_target_filename)
        .env("ENCRYPTION_KEY_PATH", encryption_key_filename)
        .env("ENCRYPTION_NONCE_PATH", nonce_filename)
        .env("TIMEOUT", timeout.to_string());

    cmd
}

pub fn compile_unpacker(
    encryption_key_filename: &String,
    nonce_filename: &String,
    compressed_input_target_filename: &String,
    target: &Option<String>,
    timeout: u64,
) -> io::Result<Vec<Artifact>> {
    let mut cmd = compile_unpacker_base_command(
        encryption_key_filename,
        nonce_filename,
        compressed_input_target_filename,
        target,
        timeout,
    );

    Ok(utils::parse_artifacts(&cmd.output()?.stdout))
}

pub fn compile_unpacker_without_aslr(
    encryption_key_filename: &String,
    nonce_filename: &String,
    compressed_input_target_filename: &String,
    target: &Option<String>,
    timeout: u64,
    unpacker_executable_path: String,
    input_image_base: u64,
    shift_rva: u64,
) -> io::Result<Vec<Artifact>> {
    let mut cmd = compile_unpacker_base_command(
        encryption_key_filename,
        nonce_filename,
        compressed_input_target_filename,
        target,
        timeout,
    );

    cmd.arg("--features")
        .arg("no-aslr")
        .env("PREVIOUS_COMPILATION", unpacker_executable_path)
        .env("INPUT_IMAGE_BASE", input_image_base.to_string())
        .env("SHIFT_RVA", shift_rva.to_string());

    Ok(utils::parse_artifacts(&cmd.output()?.stdout))
}

// use std::{
//     fs,
//     io::Write,
//     process::Output,
// };

// use cargo_metadata::{camino::Utf8PathBuf, Artifact, Target};
// use object::{
//     pe::{IMAGE_SCN_CNT_UNINITIALIZED_DATA, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE},
//     read::pe::PeFile64,
//     write::{pe::Writer, StreamingBuffer},
//     LittleEndian,
// };

// pub fn pack_artifact(
//     artifact: Artifact,
//     target: &Option<String>,
//     timeout: u64,
//     aslr_enable: bool,
// ) -> io::Result<()> {
//     let executable_path = artifact
//         .executable
//         .expect("should not fail because of previous filtering");

//     let (encryption_key_filename, nonce_filename, compressed_artifact_filename) =
//         compress_and_encrypt_artifact(&artifact.target, &executable_path)?;

//     let output = compile_unpacker_crate(
//         &encryption_key_filename,
//         &nonce_filename,
//         &compressed_artifact_filename,
//         target,
//         timeout,
//     )
//     .output()?;

//     if !aslr_enable {
//         let unpacker_executable_path = parse_artifacts(&output.stdout[..])
//             .pop()
//             .expect("there should be at least one compiler artifact")
//             .executable
//             .expect("there should be a path to the produced executable");

//         let target_bin_data = fs::read(&executable_path)?;
//         let unpacker_bin_data = fs::read(&unpacker_executable_path)?;

//         let target_pe_file = PeFile64::parse(&*target_bin_data).unwrap();
//         let unpacker_pe_file = PeFile64::parse(&*unpacker_bin_data).unwrap();

//         let unpacker_file_alignment = unpacker_pe_file
//             .nt_headers()
//             .optional_header
//             .file_alignment
//             .get(LittleEndian);
//         let unpacker_section_alignment = unpacker_pe_file
//             .nt_headers()
//             .optional_header
//             .section_alignment
//             .get(LittleEndian);

//         let target_min_RVA = target_pe_file
//             .section_table()
//             .iter()
//             .map(|s| s.virtual_address.get(LittleEndian))
//             .min()
//             .unwrap();

//         let target_max_RVA = target_pe_file
//             .section_table()
//             .iter()
//             .map(|s| s.virtual_address.get(LittleEndian) + s.virtual_size.get(LittleEndian))
//             .max()
//             .unwrap();

//         let unpacker_min_RVA = unpacker_pe_file
//             .section_table()
//             .iter()
//             .map(|s| s.virtual_address.get(LittleEndian))
//             .min()
//             .unwrap();

//         let (final_pe_filename, mut final_pe_file) = utils::generate_file_for_additional_artifact(
//             &executable_path,
//             &artifact.target,
//             "final_pe",
//         )?;

//         let mut final_pe_file_writer = StreamingBuffer::new(final_pe_file);

//         let mut final_pe = Writer::new(
//             true,
//             unpacker_section_alignment,
//             unpacker_file_alignment,
//             &mut final_pe_file_writer,
//         );

//         let alloc_section = final_pe.reserve_section(
//             *b".alloc  ",
//             IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE | IMAGE_SCN_CNT_UNINITIALIZED_DATA,
//             target_max_RVA - target_min_RVA,
//             0,
//         );

//         let shift_RVA = (target_min_RVA + alloc_section.virtual_address ) - unpacker_min_RVA;

//         let output = recompile_unpacker_crate_without_aslr(
//             &encryption_key_filename,
//             &nonce_filename,
//             &compressed_artifact_filename,
//             target,
//             timeout,
//             unpacker_executable_path.into_string(),
//         )?;

//         let recompiled_unpacker_executable_path = parse_artifacts(&output.stdout[..])
//             .pop()
//             .expect("there should be at least one compiler artifact")
//             .executable
//             .expect("there should be a path to the produced executable");
//     }
//     Ok(())
// }
