use rand::prelude::*;
use std::env;

enum Function {
    Generate,
    Sign,
    Verify,
}

fn generate_key_pair(rng: rand::prelude::ThreadRng) -> (u128, u128) {
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
/// command line arguments.  If all arguments are good, call the correct
/// function.

fn args_good(args: Vec<String>) -> Result<Function, String> {

    // ignore "0 arg", i.e. the executable name itself.
    // This means that all argument lengths here are "one more" than you
    // might expect, e.g. "./foo bar" is two arguments - "./foo" (program
    // name) and "bar" (actual argument")

    if args.len() < 2 {
        return Err("Not enough arguments".to_string());
    } else if args.len() > 5 {
        return Err("Too many arguments".to_string());
    }

    
    match args[1].as_ref() {
        "generate" => {
            if args.len() != 2 {
                return Err("generate takes no arguments".to_string());
            } else {
                return Ok(Function::Generate);
            }
        },
        "sign" => {
            if args.len() != 4 {
                return Err("sign requires two arguments".to_string());
            } else {
                return Ok(Function::Sign);
            }

        },
        "verify" => {
            if args.len() != 5 {
                return Err("verify requires three arguments".to_string());
            } else {
                return Ok(Function::Verify)                
            }

        },
        _ => {
            return Err("Unrecognized first argument".to_string());
        },
    }

}


fn main() {
    
    let mut args = Vec::new();
    for argument in env::args() {
        args.push(argument);
    }

    let args_ok = args_good(args);
    match args_ok {
        Ok(f) => {
            match f {
                Function::Generate => {
                    let mut rng = rand::thread_rng();
                    generate_key_pair(rng);
                },
                Function::Sign => {
                    // sign_message(args[2], args[3]);
                },
                Function::Verify => {
                    // verify_signature(args[2], args[3], args[4]);
                },
            }
            // let a = rand::random::<u128>();
            // println!("{:#034x}", a);
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
