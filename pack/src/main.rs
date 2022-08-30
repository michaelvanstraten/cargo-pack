// #![feature(exit_status_error)]
#![feature(seek_stream_len)]

use clap::Parser;
use pack::Pack;
use std::error::Error;

mod compilation;
mod obfuscation;
mod pack;
mod utils;

#[derive(Parser)]
struct Args {
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
