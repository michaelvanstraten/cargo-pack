use cargo_metadata::{camino::Utf8PathBuf, Artifact, Message, Target};
use rand::{rngs::OsRng, RngCore};
use std::{
    fs::File,
    io,
    ops::{Deref, DerefMut},
};

pub fn generate_key_and_nonce() -> ([u8; 32], [u8; 12]) {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);
    (key, nonce)
}

pub fn generate_file_for_additional_artifact(
    executable_path: &Utf8PathBuf,
    target: &Target,
    artifact_name: impl AsRef<str>,
) -> io::Result<NamedFile> {
    let path = format!(
        "{}/{}_{}.bytes",
        executable_path.as_path().parent().unwrap(),
        artifact_name.as_ref(),
        target.name
    );
    let file = File::create(&path)?;
    Ok(NamedFile {
        filename: path,
        file,
    })
}

pub fn parse_artifacts(output: &[u8]) -> Vec<Artifact> {
    Message::parse_stream(output)
        .into_iter()
        .filter_map(|m| match m {
            Ok(Message::CompilerArtifact(artifact)) if artifact.executable.is_some() => {
                Some(artifact)
            }
            _ => None,
        })
        .collect()
}

pub struct NamedFile {
    pub filename: String,
    file: File,
}

impl Deref for NamedFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for NamedFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}
