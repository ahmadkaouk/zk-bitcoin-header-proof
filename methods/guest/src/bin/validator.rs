#![no_main]
#![no_std]

use block_header_core::*;
use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the block to validate from the host
    let block: BlockHeader = env::read();

    let hash_u256 = U256::from_big_endian(&block.block_hash());
    // Check if block hash is invalid, if true panic
    if hash_u256 > block.target() {
        panic!("Invalid Block Header");
    }

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
