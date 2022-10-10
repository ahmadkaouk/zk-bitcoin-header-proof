# Bitcoin Block Header ZK Proof

A simple program bitcoin block header proof, built on the RISC Zero platform.

This program is implemented is rust, and uses [RISC Zero’s zkVM](https://github.com/risc0/risc0). It can be used to convince someone that we are in possission of a valid Bitcoin Block Header, whithout revealing the nonce. One might use such a gadget to convince a mining pool to give you some money up front before delivering the header.

## Overview 

The program is composed of several parts: a block validity checker (that runs in the zkVM) and a host driver (an ordinary command-line program that uses the zkVM to run the Block Validity Checker).

The block validity checker accepts a bitcoin block header from the host driver. It then uses a function that examines the validity of the block  and panics if the block is not valid. If the block is valid, execution proceeds and the zkVM appends the constituents of the block header (except for the nonce) to the journal. 

The journal is a readable record of all values committed by code in the zkVM; it is attached to the receipt (a record of correct execution). The receipt can be used by a verifier to identify which binary was executed (via the method ID), check the shared results with this particular execution (via the journal), and verify its own integrity (via the cryptographic seal).

![](resources/image.png)

## Project Organization
The program is composed from 3 main packages: core, cli and methods:
- **core**: Contains code related to the definition of the block header 
- **methods**: This package contains the block validity checker code that will run inside the ZKVM.
- **cli**: cli is a simple command line tool that calls the method in the guest and let prover validate their block and generate the receipt.
## How it works
To build and run a prover, use the following command:
```rust
cargo run --release prover --block_header <BLOCK_HEADER_HEX>
```
with BLOCK_HEADER_HEX is the is hexadecimal representation of the block header to verify.


For example:
```rust
# Valid Block Header
$ cargo run --release prover -b "0060962fd823338936a6ae879bb012979f1622b95cfae2367b41050000000000000000005d4e93622acbe5246dc22efbd6b4e9ff6f924b94ed92a56637d891bfdbb7236636944363aef908176e294224"

Receipt generated for the block BlockHeader { version: 798384128, prev_blockhash: [216, 35, 51, 137, 54, 166, 174, 135, 155, 176, 18, 151, 159, 22, 34, 185, 92, 250, 226, 54, 123, 65, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0], merkle_root: [93, 78, 147, 98, 42, 203, 229, 36, 109, 194, 46, 251, 214, 180, 233, 255, 111, 146, 75, 148, 237, 146, 165, 102, 55, 216, 145, 191, 219, 183, 35, 102], time: 1665373238, bits: 386464174, nonce: 608315758 }

# Invalid Block Header
$ cargo run --release prover -b "0060962fd823338936a6ae879bb012979f1622b95cfae2367b41050000000000000000005d4e93622acbe5246dc22efbd6b4e9ff6f924b94ed92a56637d891bfdbb7236636944363aef908176e294225"

thread '<unnamed>' panicked at 'Invalid Block Header', src/bin/validity_checker.rs:18:9
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Exception { what: "Invalid requireMux: /Users/ahmad/projects/challenges/risc_zero/bitcoin-header-proof-1-izdcuz/target/release/build/risc0-zkvm-circuit-sys-857c2bd69e9f2806/out/cxxbridge/crate/risc0/zkvm/circuit/decode_cycle.h:24" }', cli/src/main.rs:39:36
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrac
```
## TODO
- Add more unit tests to cover every possible scenario
- Add Integration and e2e tests
- Add CI/CD
- Better Error Handling
- Build a cli for verifier