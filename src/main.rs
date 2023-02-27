// Copyright 2023 Martin Pool.

//! Convert Greengate DS3 samples to WAV.

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args = std::env::args();
    assert_eq!(args.len(), 3, "usage: ds3ex IN OUTWAV");
    let input_path = std::env::args().nth(1).expect("expected a file name");
    let output_path = std::env::args().nth(2).expect("expected a file name");
    let bytes = std::fs::read(&input_path).context("read file bytes")?;

    let spec = hound::WavSpec {
        channels: 1,
        bits_per_sample: 8,
        sample_rate: 30_000,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, spec).expect("make writer");
    for b in bytes {
        // Input seems to have 0x80 as the midpoint, but wavs are signed with 0 at the midpoint.
        let s = ((b as i16) - 0x80i16) as i8;
        writer.write_sample(s).context("write sample element")?;
    }
    writer.finalize().context("finalize wav")?;
    Ok(())
}
