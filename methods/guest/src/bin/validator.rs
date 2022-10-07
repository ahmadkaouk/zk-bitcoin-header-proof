#![no_main]
#![no_std]

use risc0_zkvm_guest::{env, sha};
use block_header_zkproof_core::*;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the block to validate from the host
    let block : BlockHeader = env::read();

    // let block_hash = sha::
    // Commit Block Header constituents to the header
    env::commit({&BlockHeaderCommit {
        version: block.version,
        prev_blockhash: block.prev_blockhash,
        merkle_root: block.merkle_root,
        bits: block.bits
    }});
}
