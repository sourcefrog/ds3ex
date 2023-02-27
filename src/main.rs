// Copyright 2023 Martin Pool.

//! Convert Greengate DS3 samples to WAV.

use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Write WAVs to this dir.
    #[arg(long, short = 'o')]
    out_dir: PathBuf,

    /// DS3 sample file to read.
    #[arg(required = true)]
    input: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for input in args.input {
        let basename = input.file_name().ok_or(anyhow!("input has no filename"))?;
        print!("{}... ", basename.to_string_lossy());
        let bytes = std::fs::read(&input).context("read file bytes")?;

        let spec = hound::WavSpec {
            channels: 1,
            bits_per_sample: 8,
            sample_rate: 30_000,
            sample_format: hound::SampleFormat::Int,
        };

        let out_path = args.out_dir.join(basename).with_extension("wav");
        let mut writer = hound::WavWriter::create(&out_path, spec).expect("make writer");
        for b in bytes {
            // Input seems to have 0x80 as the midpoint, but wavs are signed with 0 at the midpoint.
            let s = ((b as i16) - 0x80i16) as i8;
            writer.write_sample(s).context("write sample element")?;
        }
        writer.finalize().context("finalize wav")?;
        println!("done!");
    }
    Ok(())
}
