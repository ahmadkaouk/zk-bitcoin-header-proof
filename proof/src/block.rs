/// A Bitcoin block header, which contains all the block information except the actual transactions
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: String,
    pub merkle_root_hash: String,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}
