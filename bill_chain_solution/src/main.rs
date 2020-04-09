use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;

const NUM_FIELD: usize = 0;
const PREV_HASH_FIELD: usize = 1;
const TXS_FIELD: usize = 2;
const TIMESTAMP_FIELD: usize = 3;
const HASH_FIELD: usize = 4;

#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64
}

fn check_valid_hash(first_line: bool, prev_line: String) -> bool {
    true
}

fn check_no_negatives(accounts: HashMap<String, u64>) -> bool {
    for (account, balance) in accounts {
        if balance < 0 {
            return false;
        }
    }
    true

}

fn check_line(accounts: &mut HashMap<String, u64>, line: String)
              -> Result<&mut HashMap<String, u64>, io::Error> {
    Ok(accounts)
}

fn parse_tx(tx: &str) -> Transaction {
    let b: Vec<&str> = tx.split(">").collect();
    let c: Vec<&str> = b[1].split("(").collect();
    let d: Vec<&str> = c[1].split(")").collect();
    let sender = b[0].to_string();
    let receiver = c[0].to_string();
    let amount = d[0].parse::<u64>().unwrap();
    Transaction { sender: sender,
                  receiver: receiver,
                  amount: amount }
}

fn coinbase(accounts: &mut HashMap<String, u64>,
            tx: Transaction) {

}

fn update_amount(accounts: &mut HashMap<String, u64>,
                 tx: Transaction) {

    if tx.sender == "SYSTEM".to_string() {
        coinbase(accounts, tx);
    } else {

        if accounts.contains_key(&tx.sender) {

            let sender_balance = accounts[&tx.sender] - tx.amount;
            accounts.insert(tx.sender, sender_balance);

            if accounts.contains_key(&tx.receiver) {
                let receiver_balance = accounts[&tx.receiver] + tx.amount;
                accounts.insert(tx.receiver, receiver_balance);
            } else {
                let receiver_balance = tx.amount;
                accounts.insert(tx.receiver, receiver_balance);
            }
        } else {
            println!("invalid tx");
        }
    }

}

fn update_accounts(fields: Vec<&str>, mut accounts: &mut HashMap<String, u64>) {
    let tx_field = fields[TXS_FIELD];
    let txs: Vec<&str> = tx_field.split(":").collect();
    for tx in txs {
        let r = parse_tx(tx);
        println!("{:?}", r);
        update_amount(&mut accounts, r);
    }
}

fn parse_file(contents: String) -> HashMap<String, u64> {
    let mut accounts = HashMap::new();
    let lines = contents.lines();
    for line in lines {
        println!("{}", line);
        let mut split = line.split("|");
        let fields: Vec<&str> = split.collect();
        for f in &fields {
            println!("\t{}", f);
        }
        update_accounts(fields, &mut accounts);
    }
    accounts
}

fn print_final_results(results: HashMap<String, u64>) {
    for (account, balance) in &results {
        println!("{}: {}", account, balance);
    }
}

/// Simple function to tell the user about appropriate usage and exit with exit code 1.
fn print_usage_and_exit() {
    println!("Please enter one and only one argument");
    std::process::exit(1);
}


fn get_file_name() -> String {
    let mut args = Vec::new();
    for argument in env::args() {
        args.push(argument);
    }

    // ignore "0 arg", i.e. the executable name itself
    let args_len = args.len() - 1;

    if args_len != 1 {
        print_usage_and_exit();
    }

    args[1].clone()

}

fn read_in_file(file_name: &String) -> Result<String, io::Error> {
    let file = File::open(get_file_name())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let file_name = get_file_name();
    let contents = read_in_file(&file_name).expect("Couldn't read file");
    let results = parse_file(contents);
    print_final_results(results);
}

// Unit tests begin here

#[cfg(test)]
mod tests {
    use super::*;
}
