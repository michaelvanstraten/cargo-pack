use std::{
    io::{Result as IOResult, Write},
    ops::DerefMut,
};

use cargo_metadata::{camino::Utf8PathBuf, Artifact, Target};

use crate::{
    compilation::{compile_unpacker, compile_unpacker_without_aslr},
    obfuscation::encrypt_and_compress_large_file,
    utils::{self, NamedFile},
};

pub struct Pack {
    input_target: Target,
    input_executable_path: Utf8PathBuf,
    encryption_key_file: NamedFile,
    nonce_file: NamedFile,
    compressed_input_target_file: NamedFile,
}

impl Pack {
    pub fn setup(artifact: Artifact) -> IOResult<Self> {
        let input_executable_path = artifact
            .executable
            .expect("should not fail because of previous filtering");
        let input_target = artifact.target;

        Ok(Self {
            encryption_key_file: utils::generate_file_for_additional_artifact(
                &input_executable_path,
                &input_target,
                "encryption_key",
            )?,
            nonce_file: utils::generate_file_for_additional_artifact(
                &input_executable_path,
                &input_target,
                "nonce",
            )?,
            compressed_input_target_file: utils::generate_file_for_additional_artifact(
                &input_executable_path,
                &input_target,
                "compressed_crate",
            )?,
            input_executable_path,
            input_target,
        })
    }

    pub fn compile(
        &mut self,
        targe_platform: &Option<String>,
        timeout: u64,
        disable_aslr: bool,
    ) -> IOResult<Artifact> {
        let (key, nonce) = utils::generate_key_and_nonce();
        self.encryption_key_file.write(&key)?;
        self.nonce_file.write(&nonce)?;

        encrypt_and_compress_large_file(
            self.input_executable_path.as_str(),
            self.compressed_input_target_file.deref_mut(),
            &key,
            &nonce,
        )?;

        let target_artifact = compile_unpacker(
            &self.encryption_key_file.filename,
            &self.nonce_file.filename,
            &self.compressed_input_target_file.filename,
            targe_platform,
            timeout,
        )?
        .pop()
        .expect("there should be at least one compiler artifact");

        if disable_aslr {
            self.recompile_without_aslr(target_artifact, targe_platform, timeout)
        } else {
            Ok(target_artifact)
        }
    }

    fn recompile_without_aslr(
        &mut self,
        previos_compilation: Artifact,
        targe_platform: &Option<String>,
        timeout: u64,
    ) -> IOResult<Artifact> {
        let input_image_base = 8192u64;
        let shift_rva = 4096u64;

        let final_artifact = compile_unpacker_without_aslr(
            &self.encryption_key_file.filename,
            &self.nonce_file.filename,
            &self.compressed_input_target_file.filename,
            targe_platform,
            timeout,
            previos_compilation
                .executable
                .expect("there should be at least one compiler artifact")
                .to_string(),
            input_image_base,
            shift_rva,
        )?
        .pop()
        .expect("there should be at least one compiler artifact");

        Ok(final_artifact)
    }
}
