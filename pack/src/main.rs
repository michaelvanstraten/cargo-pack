#![feature(seek_stream_len)]
#![feature(let_chains)]
/*!
Cargo-pack is an executable packer for rust crates. It allows you to easy pack any type of rust crate that has a binary target.

# Features
- binary encryption/opfuscation using `ChaCha20`
- compression of target using `brotli` stream compression algorithm

# Install
To install this crate you first have to clone the repository.
The crate compiles a target by wrapping it inn a wrapper crate, which is located in this repository.
In order to find the crate, after you run `cargo install` the location of the repository can't move.
`git clone https://github.com/michaelvanstraten/cargo-pack`

*/

mod compilation;
mod obfuscation;
mod pack;
mod utils;

use std::error::Error;

use clap::Parser;
use pack::Pack;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    is_called_by_cargo: (),
    #[clap(long = "target")]
    target_platform: Option<String>,
    #[clap(short = 'w', long = "timeout", default_value = "300")]
    timeout: u64,
    #[clap(last = true)]
    cargo_args: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let to_pack_artifacts =
        compilation::compile_input_target(&args.target_platform, args.cargo_args)?;

    for artifact in to_pack_artifacts {
        let mut pack = Pack::setup(artifact)?;
        pack.compile(&args.target_platform, args.timeout, false)?;
    }

    Ok(())
}
