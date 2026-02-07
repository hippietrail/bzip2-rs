//! Test block offset tracking

use bzip2_rs::decoder::{Decoder, ReadState};

#[test]
fn test_block_offsets_sample1() {
    let compressed_file: &[u8] = include_bytes!("samplefiles/sample1.bz2").as_ref();
    let mut decoder = Decoder::new();

    let mut input = compressed_file;
    let mut buf = [0; 1024];
    let mut total_decompressed = 0;

    loop {
        match decoder.read(&mut buf).expect("should not error") {
            ReadState::NeedsWrite => {
                if !input.is_empty() {
                    let chunk_size = std::cmp::min(256, input.len());
                    decoder.write(&input[..chunk_size]);
                    input = &input[chunk_size..];
                } else {
                    // Signal EOF
                    decoder.write(&[]);
                }
            }
            ReadState::Read(n) => {
                total_decompressed += n;
            }
            ReadState::Eof => {
                break;
            }
        }
    }

    // Check that we recorded at least one block offset
    let offsets = decoder.block_offsets();
    println!(
        "Recorded {} blocks, total decompressed {} bytes",
        offsets.len(),
        total_decompressed
    );
    for (i, offset) in offsets.iter().enumerate() {
        println!(
            "  Block {}: compressed_bit_offset={}, decompressed_byte_offset={}",
            i, offset.compressed_bit_offset, offset.decompressed_byte_offset
        );
    }
    
    assert!(
        !offsets.is_empty(),
        "Should have recorded at least one block offset"
    );

    // First block should start at bit offset 32 (after 4-byte BZh9 header)
    assert_eq!(
        offsets[0].compressed_bit_offset, 32,
        "First block should start after 4-byte header, got {}",
        offsets[0].compressed_bit_offset
    );
    assert_eq!(
        offsets[0].decompressed_byte_offset, 0,
        "First block decompressed bytes should start at 0"
    );

    // Verify we got correct decompressed output
    let expected_decompressed: &[u8] =
        include_bytes!("samplefiles/sample1.ref").as_ref();
    assert_eq!(
        total_decompressed, expected_decompressed.len(),
        "Should decompress correct number of bytes"
    );
}

#[test]
fn test_block_offsets_all_samples() {
    // Test that all sample files produce reasonable block offsets
    let samples = [
        ("sample1.bz2", include_bytes!("samplefiles/sample1.bz2").as_ref()),
        ("sample2.bz2", include_bytes!("samplefiles/sample2.bz2").as_ref()),
        ("sample3.bz2", include_bytes!("samplefiles/sample3.bz2").as_ref()),
    ];

    for (name, compressed_file) in &samples {
        let mut decoder = Decoder::new();
        let mut input = *compressed_file;
        let mut buf = [0; 1024];

        loop {
            match decoder.read(&mut buf).expect("should not error") {
                ReadState::NeedsWrite => {
                    if !input.is_empty() {
                        let chunk_size = std::cmp::min(256, input.len());
                        decoder.write(&input[..chunk_size]);
                        input = &input[chunk_size..];
                    } else {
                        decoder.write(&[]);
                    }
                }
                ReadState::Read(_n) => {}
                ReadState::Eof => {
                    break;
                }
            }
        }

        let offsets = decoder.block_offsets();
        println!("{}: {} blocks", name, offsets.len());
        for (i, offset) in offsets.iter().enumerate() {
            println!(
                "  Block {}: compressed_bit={}, decompressed_byte={}",
                i, offset.compressed_bit_offset, offset.decompressed_byte_offset
            );
        }

        // Every file should have at least one block
        assert!(
            !offsets.is_empty(),
            "{} should have at least one block",
            name
        );

        // First block should start at bit offset 32 (after 4-byte BZh header)
        assert_eq!(
            offsets[0].compressed_bit_offset, 32,
            "{}: first block should start at offset 32",
            name
        );

        // First block decompressed offset should be 0
        assert_eq!(
            offsets[0].decompressed_byte_offset, 0,
            "{}: first block decompressed offset should be 0",
            name
        );

        // Verify offsets are strictly increasing
        for i in 0..offsets.len() - 1 {
            assert!(
                offsets[i].compressed_bit_offset < offsets[i + 1].compressed_bit_offset,
                "{}: Compressed offsets should be strictly increasing",
                name
            );
            assert!(
                offsets[i].decompressed_byte_offset <= offsets[i + 1].decompressed_byte_offset,
                "{}: Decompressed offsets should be non-decreasing",
                name
            );
        }
    }
}
