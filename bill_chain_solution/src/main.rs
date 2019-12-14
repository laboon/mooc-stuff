use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;

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

fn check_valid_num_billcoins(line: String) -> bool {
    true
}

fn check_line(accounts: &mut HashMap<String, u64>, line: String)
              -> Result<&mut HashMap<String, u64>, io::Error> {
    Ok(accounts)
}

fn parse_file(contents: String) -> HashMap<String, u64> {
    let mut accounts = HashMap::new();
    let lines = contents.lines();
    for line in lines {
        println!("{}", line);
    }
    accounts
}

fn print_final_results(results: HashMap<String, u64>) {
    for (account, balance) in &results {
        println!("{}: \"{}\"", account, balance);
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
    let file = File::open("foo.txt")?;
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
