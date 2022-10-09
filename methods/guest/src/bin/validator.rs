#![no_main]
#![no_std]

use block_header_core::*;
use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the block to validate from the host
    let block: BlockHeader = env::read();

    // Compute the hash of the block header
    // let block_hash = sha::raw_digest(sha::digest_u8_slice(&block.block_hash()));
    let target = block.target();

    // Commit Block Header constituents to the header
    env::commit({
        &BlockHeaderCommit {
            version: block.version,
            prev_blockhash: block.prev_blockhash,
            merkle_root: block.merkle_root,
            time: block.time,
            bits: block.bits,
        }
    });
}
