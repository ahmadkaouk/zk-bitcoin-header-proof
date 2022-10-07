use serde::{Deserialize, Serialize};

/// A Bitcoin block header, which contains all the block information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlockHeader {
    pub version: i32,
    pub prev_blockhash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    /// Get Little Endian bytes representation of the block
    pub fn to_le_bytes(&self) -> [u8; 80] {
        [
            self.version.to_le_bytes().into(),
            self.prev_blockhash.into_iter().rev().collect::<Vec<_>>(),
            self.merkle_root.into_iter().rev().collect::<Vec<_>>(),
            self.time.to_le_bytes().into(),
            self.bits.to_le_bytes().into(),
            self.nonce.to_le_bytes().into(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Cannot extract bytes from Block Header")
    }

    /// Compute target from bits
    /// formula : target = coefficient * 2 ^ (8 * (index — 3))
    pub fn target(&self) -> [u64; 4] {
        // TODO Compute target from bits
        [0; 4]
    }
}

/// Contains Block Header Informations to be commited to the journal
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlockHeaderCommit {
    pub version: i32,
    pub prev_blockhash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub time: u32,
    pub bits: u32,
}
