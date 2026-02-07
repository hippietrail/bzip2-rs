//! Block offset tracking for bzip2 streams
//!
//! This module provides APIs to track where bzip2 blocks occur
//! in both the compressed bitstream and decompressed output.

/// A mapping from compressed position to decompressed position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockOffset {
    /// Bit offset in the compressed stream where this block starts
    pub compressed_bit_offset: u64,
    /// Byte offset in the decompressed stream where this block starts
    pub decompressed_byte_offset: u64,
}

/// Collects block offsets during decompression
#[derive(Debug, Clone, Default)]
pub struct BlockOffsetCollector {
    offsets: Vec<BlockOffset>,
}

impl BlockOffsetCollector {
    /// Create a new, empty offset collector
    pub fn new() -> Self {
        Self {
            offsets: Vec::new(),
        }
    }

    /// Record a block offset
    pub(crate) fn record(&mut self, compressed_bit_offset: u64, decompressed_byte_offset: u64) {
        self.offsets.push(BlockOffset {
            compressed_bit_offset,
            decompressed_byte_offset,
        });
    }

    /// Get all recorded block offsets
    pub fn offsets(&self) -> &[BlockOffset] {
        &self.offsets
    }

    /// Clear all recorded offsets
    pub fn clear(&mut self) {
        self.offsets.clear();
    }

    /// Get the number of blocks recorded
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    /// Check if no blocks have been recorded
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_offset_collector() {
        let mut collector = BlockOffsetCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.len(), 0);

        collector.record(0, 0);
        assert!(!collector.is_empty());
        assert_eq!(collector.len(), 1);
        assert_eq!(collector.offsets()[0].compressed_bit_offset, 0);
        assert_eq!(collector.offsets()[0].decompressed_byte_offset, 0);

        collector.record(1024, 64000);
        assert_eq!(collector.len(), 2);
        assert_eq!(collector.offsets()[1].compressed_bit_offset, 1024);
        assert_eq!(collector.offsets()[1].decompressed_byte_offset, 64000);

        collector.clear();
        assert!(collector.is_empty());
    }
}
