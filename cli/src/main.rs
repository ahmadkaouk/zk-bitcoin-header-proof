use std::fs;

use block_header_core::BlockHeader;
use block_header_validity_methods::{VALIDITY_CHECKER_ID, VALIDITY_CHECKER_PATH};
use clap::{arg, command, Command};
use risc0_zkvm::{host::Prover, serde::to_vec};

fn main() {
    let matches = command!()
        .about("A program to prove the validity of a block header with RISC0")
        .subcommand(
            Command::new("prover")
                .about("Prover to produce a receipt for a valid block header")
                .arg(
                    arg!(-b --block_header <BLOCK_HEADER_HEX> ... "Block header in hex representation")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("prover") {
        // Read the block header in hex representation
        let block_hex = matches.get_one::<String>("block_header").expect("required");
        let block: BlockHeader = bincode::deserialize(&hex::decode(block_hex).unwrap()).unwrap();

        // A prover is created to run the validity checker method
        let mut prover = Prover::new(
            &fs::read(VALIDITY_CHECKER_PATH).unwrap(),
            VALIDITY_CHECKER_ID,
        )
        .unwrap();

        // Adding input to the prover makes it readable by the guest
        let vec = to_vec(&block).unwrap();
        prover.add_input(&vec).unwrap();

        let receipt = prover.run().unwrap();
        receipt
            .verify(VALIDITY_CHECKER_ID)
            .expect("Block is not valid");

        println!("Receipt generated for the block {:?}", block);
    }
}
