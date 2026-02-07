//! Validate block offsets against seek-table tool output

use bzip2_rs::decoder::{Decoder, ReadState};
use std::process::Command;

/// Helper to run seek-table and parse output
fn get_seek_table_offsets(file_path: &str) -> Vec<(u64, u64)> {
    let output = Command::new("seek-table")
        .arg(file_path)
        .output()
        .expect("failed to run seek-table");

    if !output.status.success() {
        panic!("seek-table failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let stdout = String::from_utf8(output.stdout).expect("invalid utf8 from seek-table");
    let mut offsets = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let bit_offset = parts[0].parse::<u64>().expect("parse bit offset");
            let size = parts[1].parse::<u64>().expect("parse size");
            offsets.push((bit_offset, size));
        }
    }

    offsets
}

/// Helper to decompress and collect block offsets
fn get_decoder_offsets(file_path: &str) -> Vec<(u64, u64)> {
    let compressed = std::fs::read(file_path).expect("read file");
    let mut decoder = Decoder::new();
    let mut input = compressed.as_slice();
    let mut buf = [0; 8192];
    let mut decompressed_sizes = Vec::new();
    let mut last_offset = 0u64;

    loop {
        match decoder.read(&mut buf).expect("decode error") {
            ReadState::NeedsWrite => {
                if !input.is_empty() {
                    let chunk = std::cmp::min(4096, input.len());
                    decoder.write(&input[..chunk]);
                    input = &input[chunk..];
                } else {
                    decoder.write(&[]);
                }
            }
            ReadState::Read(n) => {
                last_offset += n as u64;
            }
            ReadState::Eof => break,
        }
    }

    // Convert block offsets to (bit_offset, block_size) pairs
    let offsets = decoder.block_offsets();
    for (i, offset) in offsets.iter().enumerate() {
        let size = if i + 1 < offsets.len() {
            offsets[i + 1].decompressed_byte_offset - offset.decompressed_byte_offset
        } else {
            last_offset - offset.decompressed_byte_offset
        };
        decompressed_sizes.push((offset.compressed_bit_offset, size));
    }

    decompressed_sizes
}

#[test]
fn test_block_offsets_match_seek_table() {
    let test_files = [
        "tests/samplefiles/sample1.bz2",
        "tests/samplefiles/sample2.bz2",
        "tests/samplefiles/sample3.bz2",
    ];

    for file_path in &test_files {
        println!("\nValidating {}", file_path);

        let seek_table_offsets = get_seek_table_offsets(file_path);
        let decoder_offsets = get_decoder_offsets(file_path);

        println!("  seek-table: {:?}", seek_table_offsets);
        println!("  decoder:    {:?}", decoder_offsets);

        assert_eq!(
            seek_table_offsets.len(),
            decoder_offsets.len(),
            "{}: Block count mismatch",
            file_path
        );

        for (i, (expected, actual)) in seek_table_offsets.iter().zip(decoder_offsets.iter()).enumerate() {
            assert_eq!(
                expected.0, actual.0,
                "{}: Block {}: compressed bit offset mismatch",
                file_path, i
            );
            assert_eq!(
                expected.1, actual.1,
                "{}: Block {}: decompressed block size mismatch",
                file_path, i
            );
        }
    }
}
