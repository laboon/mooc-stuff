//! A Basic Blockchain

use std::collections::HashMap;
use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::prelude::*;
use std::io;

// For simplicity, every block will have exactly one transaction - for efficiency,
// you will generally see 0..n hashes in a block.
// A transaction consists of a "to" address, a "from" address, and amount sent
// A block contains a transaction and the hash of the previous block
// The Debug trait just lets us easily print it out using println!
// The Hash trait allows us to hash a struct of this type

#[derive(Debug, Hash)]
pub struct Block {
    pub to_addr: u64,
    pub from_addr: u64,
    pub amount: u64,
    pub prev_hash: u64
}


// Given any object, return its 64-bit hash.  This uses the default
// Rust hashing algorithm.

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let r = s.finish();
    r 
}

fn pretty_print_block(b: &Block) {
    print!("{:#016x} sent {} to {:#016x} (Prev Hash: {:#016x})",
           b.from_addr,
           b.amount,
           b.from_addr,
           b.prev_hash);
}

// fn generate_block(from_addr: u64,
//                   to_addr: u64,
//                   amount: u64,
//                   prev_hash: u64) -> Block {
//     let new_block = Block {
//         from_addr: from_addr,
//         to_addr: to_addr,
//         amount: amount,
//         prev_hash: prev_hash
//     };
//     new_block
// }

fn print_blockchain(bc: Vec<Block>) {
    for (j, b) in bc.iter().enumerate() {
        println!("{},{:#016x},{},{:#016x},{:#016x}",
                 j,
                 b.from_addr,
                 b.amount,
                 b.to_addr,
                 b.prev_hash);
        
    }
}

fn print_results(results: HashMap<u64, u64>) {
    for (address, amount) in &results {
        println!("{} : {} billcoins", address, amount);
    }
}

fn read_file() -> Result<Vec<String>, String> {
    Err("derp".to_string())
}

fn parse_file() -> Result<Vec<Block>, String> {
    Err("derp".to_string())
}
fn verify_file() -> Result<HashMap<u64, u64>, String> {
    Err("derp".to_string())
}

fn read_blockchain(f: String) -> Result<HashMap<u64, u64>, String> {
    Err("derp".to_string())
}

fn convert_hex(x: String) -> u64 {
    let num = x.trim_start_matches("0x");
    u64::from_str_radix(num, 16).unwrap()
}
                             
fn convert_decimal(x: String) -> u64 {
    u64::from_str_radix(&x, 10).unwrap()
}

fn get_block_info(prev_hash: u64) -> Option<Block> {
    let mut to_addr: String = String::new();
    let mut from_addr: String = String::new();
    let mut amount: String = String::new();

    print!("From address (hex) > ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut from_addr).unwrap();
    from_addr = from_addr.trim().to_string();
    if from_addr == "x" {
        return None
    }
    print!("To address (hex) > ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut to_addr).expect("Error");
    to_addr = to_addr.trim().to_string();
    
    print!("Amount > ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut amount).expect("Error");
    amount = amount.trim().to_string();

    // Generate block from input

    let b = Block {
        to_addr: convert_hex(to_addr),
        from_addr: convert_hex(from_addr),
        amount: convert_decimal(amount),
        prev_hash: prev_hash
    };

    Some(b)
        
}
fn make_blockchain() -> Vec<Block> {
    let mut prev_hash = 0;
    
    let mut blockchain: Vec<Block> = Vec::new();
    let mut cont = true;

    let mut block_num = 0;
    while (cont) {
        println!("Block Number: {}", block_num);
        
        let block_option = get_block_info(prev_hash);
        match block_option {
            Some(b) => {
                // Get hash of this block to use as prev_hash for
                // NEXT block
                prev_hash = get_hash(&b);

                // Add block to blockchain
                blockchain.push(b);
            },
            None => {
                // Stop collecting blocks from user
                break;
            }
        }
        
        // // Reset strings.  Otherwise, read_line will concatenate
        // // strings from iteration to iteration of the loop.
        // to_addr = "".to_string();
        // from_addr = "".to_string();
        // amount = "".to_string();

        block_num = block_num + 1;

    }

    blockchain
    
}
fn print_usage_and_exit() {
    println!("Usage:");
    println!("No arguments: ");
    println!("One argument: Read file specified by argument and display if blockchain is valid");
    std::process::exit(1);
}

fn main() {

    let args_count = env::args().count();
    if args_count <= 1 {
        let blockchain = make_blockchain();
        print_blockchain(blockchain);
    } else if args_count == 2 {
        // Note: we know this element exists, otherwise we would
        // have to worry about unwrap() panicking
        let valid = read_blockchain(env::args().nth(1).unwrap());
        match valid {
            Ok(bc) => {
                print_results(bc);
                println!("Blockchain valid!");
            },
            Err(e) => {
                println!("Blockchain invalid: {}", e);
            }
        } 
            
          
    } else {
        print_usage_and_exit();
    }

    // let x1 = generate_block(98, 87, 99, 0);
    // let x2 = generate_block(99, 8, 3982, get_hash(&x1));
    // pretty_print_block(&x1);
    // println!("");
    // pretty_print_block(&x2);
    // println!("");
    
    // println!("{:?}", x);
    // println!("{:?}", get_hash(&x));
}

#[cfg(test)]
mod tests {
    use super::*;
}
