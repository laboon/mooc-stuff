use rand::prelude::*;
use std::env;


fn generate_key_pair(seed: u128) -> (u128, u128) {
    (0, 0)
}

fn sign_message(msg: String, priv_key: u128) -> u128 {
    1
}

fn verify_signature(msg: String, sig: String, pub_key: u128) -> bool {
    false
}

/// Simple function to tell the user about appropriate usage and exit with exit code 1.
fn print_usage_and_exit() {
    println!("Usage:");
    println!("generate - generates a public/private keypair");
    println!("sign <msg> <private_key> - signs a message with private key");
    println!("verify <msg> <signature> <public_key> - verifies a message");
    std::process::exit(1);
}


/// "Wrapper function" which returns the string to hash, getting it from the
/// command line arguments.  It also does some simple housekeeping to ensure
/// that a single argument was passed in, and exits if not.

fn args_good(args: Vec<String>) -> Result<String, String> {

    // ignore "0 arg", i.e. the executable name itself
    let args_len = args.len() - 1;

    if args_len == 0 {
        return Err("Not enough arguments".to_string());
    }

    
    match args[1].as_ref() {
        "generate" => {
            println!("g");
        },
        "sign" => {
            println!("s");

        },
        "verify" => {
            println!("v");

        },
        _ => {
            return Err("Unrecognized first argument".to_string());
        },
    }
    // if args[0] == "
    
    // args[1].clone()
    Ok("meow".to_string())
}


fn main() {
    
    let mut args = Vec::new();
    for argument in env::args() {
        args.push(argument);
    }

    let args_ok = args_good(args);
    match args_ok {
        Ok(_) => {
            let a = rand::random::<u128>();
            println!("{:#034x}", a);
        },
        Err(e) => {
            println!("Error: {}", e);
            print_usage_and_exit();
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

// 0x45e345cc30b0f72fb7f41fb4afd58d5b
