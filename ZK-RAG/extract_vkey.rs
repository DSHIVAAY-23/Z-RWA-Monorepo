use sp1_sdk::{ProverClient, HashableKey};
use std::fs;

fn main() {
    let elf_path = "crates/circuits/elf/riscv32im-succinct-zkvm-elf";
    let client = ProverClient::new();
    let (_, vk) = client.setup(&fs::read(elf_path).unwrap());
    println!("{}", vk.bytes32());
}
