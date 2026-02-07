# Block Offset Tracking Implementation Notes

## Current Implementation (feature/block-offsets)

### API
```rust
// On Decoder
pub fn block_offsets(&self) -> &[BlockOffset] { ... }
pub fn clear_block_offsets(&mut self) { ... }

// On DecoderReader
pub fn block_offsets(&self) -> &[BlockOffset] { ... }
pub fn clear_block_offsets(&mut self) { ... }

pub struct BlockOffset {
    pub compressed_bit_offset: u64,
    pub decompressed_byte_offset: u64,
}
```

### How It Works

1. **Bit Position Tracking**: The decoder maintains `total_bits_consumed` which tracks cumulative bits read from the input stream.

2. **Block Detection**: When a block transitions from `ReadyForRead` → `Reading` state:
   - Record the current `compressed_bit_offset` (calculated as `total_bits_consumed + reader.position()`)
   - Record the current `decompressed_byte_offset`

3. **Reset Mechanism**: The `last_block_recorded` flag prevents duplicate recording within a single block. It's reset when transitioning `NotReady` → `ReadyForRead` (when preparing to read the next block).

4. **Output Format**: Currently provides `(bit_offset, cumulative_decompressed_offset)` pairs.

### Key Design Decisions

- **Sequential Processing**: Offsets are recorded as blocks are encountered, no separate scanning phase
- **No Buffering Required**: Works during single-pass decompression
- **Seek-table Compatible**: Output format matches the compressed_bit_offset used by seek-bzip and seek-table tools

## Validation

Uses `seek-table` tool (Node.js/JavaScript) as ground truth:
```bash
$ seek-table file.bz2
32      98696      # compressed_bit_offset  decompressed_block_size
544888  11550
```

Our implementation correctly produces matching offsets for all test files.

## Next Steps: Seek-Table Compatible Format

The seek-table format outputs `(compressed_bit_offset, decompressed_block_size)` per block.
Current implementation provides cumulative decompressed offsets instead of per-block sizes.

### Conversion Strategy
After decompression completes, compute block sizes from cumulative offsets:
```rust
for i in 0..offsets.len() {
    let size = if i + 1 < offsets.len() {
        offsets[i+1].decompressed_byte_offset - offsets[i].decompressed_byte_offset
    } else {
        total_decompressed - offsets[i].decompressed_byte_offset
    };
}
```

This is "Option 1: Post-processing" from the plan - simplest approach.

## Known Limitations

- **ParallelDecoder**: Not yet integrated (uses different architecture with separate scanning phase)
- **Multistream Files**: Not tested yet (concatenated bzip2 streams); lower priority
- **Block Sizes**: Currently requires post-processing to convert from cumulative offsets

## References

Old C implementation in seek-bzip (found on external drive):
- `/Volumes/DYNABOOK/hippietrail from M1 Mac/bzip stuff/seek-bzip/bzip-table.c`
- ~94 lines of C showing bit-position tracking
- Same output format: `bit_offset<tab>decompressed_block_size`
