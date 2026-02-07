# Multistream bzip2 Testing Notes

## Current Status

Multistream bzip2 files (concatenated bzip2 streams) are **not yet tested** with the block offset tracking implementation.

## What Are Multistream Files?

Multistream bzip2 files are created by concatenating multiple independent bzip2 streams:
```bash
bzip2 file1.txt > file1.bz2
bzip2 file2.txt > file2.bz2
cat file1.bz2 file2.bz2 > combined.bz2  # This is a multistream file
```

Each stream has:
- Its own BZh header (4 bytes)
- Its own sequence of blocks with π markers
- Its own √π end-of-stream marker

## Implementation Status

### ✓ Should Work
- Decoder reads blocks sequentially, so it should naturally handle multistream files
- Each stream starts with a fresh BZh header, which Decoder handles
- Block offset recording should continue across stream boundaries

### ? Needs Testing
- Verify offsets are correct across stream boundaries
- Ensure `total_bits_consumed` tracking works correctly at stream boundaries
- Confirm decompression offset tracking remains accurate

## Validation Challenge

The `seek-table` tool (on PATH) **does not properly handle multistream files**:
- Only reports the first stream's block offsets
- Limitation in the tool itself, not our implementation
- Makes ground-truth validation impossible with this tool

### Workaround Options

1. **Manual Verification**
   - Create test multistream file
   - Decompress with `bzip2 -d` to verify correctness
   - Manually inspect with `itty_bitty` tool for bit offsets

2. **Alternative Tools**
   - Research if other `seek-table` implementations handle multistream
   - Look for `seek-bzip` tool variants with multistream support
   - Check bzip2 utilities documentation

3. **Implement Custom Validation**
   - Create test that decompresses with our decoder
   - Verify by re-reading at recorded offsets
   - Cross-check decompressed output integrity

## Testing Plan (Low Priority)

Since multistream files are not the primary use case (and typically larger than equivalent single-stream files), this is marked low priority:

1. Create test multistream file from existing samples
2. Run decoder and collect offsets
3. Manually verify a few offsets with `itty_bitty`
4. Confirm decompression is correct
5. Document any issues found

## References

- Standard bzip2 supports multistream via concatenation
- Most real-world uses single-stream (more efficient)
- seek-bzip/seek-table tools may have different implementations with better multistream support
- Old external drive backup may have notes on this (check `/Volumes/DYNABOOK/hippietrail from M1 Mac/bzip stuff/`)
