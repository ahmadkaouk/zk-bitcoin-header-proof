use primitive_types::U256;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
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
    /// Compute the hash of the block header.
    pub fn block_hash(&self) -> Vec<u8> {
        // Step 1 Transform block into little endian hexadecimal (in form of bytes)
        let block_le = bincode::serialize(&self).unwrap();

        // Step 2 Apply a SHA-256 on the binary representation of the litte endian hex obtained in step 1
        let mut hasher = Sha256::new();
        hasher.update(block_le);
        let hash1 = hasher.finalize();

        // Step 3 Apply a SHA-256 on the hash obtained in step 2
        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let mut hash = hasher.finalize().to_vec();
       
        // Step 4 Convert the result into little endian 
        hash.reverse();
        hash
    }

    /// Compute target from bits
    /// formula : target = coefficient * 2 ^ (8 * (index — 3))
    pub fn target(&self) -> U256 {
        let (target, expt) = {
            let unshifted_expt = self.bits >> 24;
            if unshifted_expt <= 3 {
                (
                    (self.bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)),
                    0,
                )
            } else {
                (self.bits & 0xFFFFFF, 8 * ((self.bits >> 24) - 3))
            }
        };

        if target > 0x7FFFFF {
            Default::default()
        } else {
            U256::try_from(target as u64).unwrap() << (expt as usize)
        }
    }
}

/// Contains Block Header informations to be commited to the journal
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlockHeaderCommit {
    pub version: i32,
    pub prev_blockhash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub time: u32,
    pub bits: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;

    #[test]
    fn test_valid_block_hex() {
        let block_hex = "010000009500c43a25c624520b5100adf82cb9f9da72fd2447a496bc600b0000000000006cd862370395dedf1da2841ccda0fc489e3039de5f1ccddef0e834991a65600ea6c8cb4db3936a1ae3143991";

        // Parse the hex data into a block header value
        let b: BlockHeader = bincode::deserialize(&hex::decode(block_hex).unwrap()).unwrap();

        assert_eq!(
            b,
            BlockHeader {
                version: 1,
                prev_blockhash: Vec::from_hex(
                    "9500c43a25c624520b5100adf82cb9f9da72fd2447a496bc600b000000000000"
                )
                .unwrap()
                .try_into()
                .unwrap(),
                merkle_root: Vec::from_hex(
                    "6cd862370395dedf1da2841ccda0fc489e3039de5f1ccddef0e834991a65600e"
                )
                .unwrap()
                .try_into()
                .unwrap(),
                time: 1305200806,
                bits: 443192243,
                nonce: 2436437219
            }
        )
    }

    #[test]
    fn test_block_hash() {
        let block_hex = "010000009500c43a25c624520b5100adf82cb9f9da72fd2447a496bc600b0000000000006cd862370395dedf1da2841ccda0fc489e3039de5f1ccddef0e834991a65600ea6c8cb4db3936a1ae3143991";
        // Parse the hex data into a block header value
        let block: BlockHeader = bincode::deserialize(&hex::decode(block_hex).unwrap()).unwrap();
        assert_eq!(
            block.block_hash(),
            hex::decode("0000000000002917ed80650c6174aac8dfc46f5fe36480aaef682ff6cd83c3ca")
                .unwrap()
        );
    }
    #[test]
    fn test_block_target_from_bits() {
        let block_hex = "010000009500c43a25c624520b5100adf82cb9f9da72fd2447a496bc600b0000000000006cd862370395dedf1da2841ccda0fc489e3039de5f1ccddef0e834991a65600ea6c8cb4db3936a1ae3143991";
        let block: BlockHeader = bincode::deserialize(&hex::decode(block_hex).unwrap()).unwrap();

        assert_eq!(block.target(), U256::from_str_radix("0x6a93b30000000000000000000000000000000000000000000000", 16).unwrap());
    }
}
