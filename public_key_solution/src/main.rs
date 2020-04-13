extern crate crypto;


use rand::prelude::*;
use std::env;
use std::process::*;
use num::integer::*;
use modinverse::modinverse;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

// Our two keys can not be higher than this value
// This makes cracking the code relatively simple, but frees us
// from having to use bigint everywhere for multiplication!
const MAX_KEY_VAL: u32 = 65_536;

// The different functions supported by the program -
// 1. Generate a keypair
// 2. Sign a message
// 3. Verify a signature against a message
enum Function {
    Generate,
    Sign,
    Verify,
}

// ****************************************************************
// Helper functions
// ****************************************************************


/// Check primality of a given unsigned integer
/// This is a pretty straightforward implementation of the `6k +/- 1` trial
/// division primality test described here:
/// https://en.wikipedia.org/wiki/Primality_test#Simple_methods.

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while (i * i <= n) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
                
    return true;
        
}

/// This function will return a random prime.
/// It does this by randomly generating an integer and testing if it's
/// prime.  There are definitely more efficient algorithms for this,
/// but this is meant to be as simple as possible.

fn get_random_prime(rng: &mut rand::prelude::ThreadRng) -> u32 {

    // Generate a random 16-bit unsigned integer.
    let mut p: u32 = 0; 

    // Keep generating random numbers and putting them in `p` until
    // the generated number is found to be prime.
    // Note that Rust does not have a do...while equivalent, so this
    // break statement may seem strange if you are coming from a different
    // language.
    loop {
        
        p = rng.gen_range(3, MAX_KEY_VAL);

        if is_prime(p) {
            break;
        }

    }

    // Return the last generated number, which should be prime
    p

}

fn is_coprime(x: u32, y: u32) -> bool {
    num::integer::gcd(x, y) == 1
}

fn carmichael_totient(x: u32, y: u32) -> u32 {
    num::integer::lcm(x - 1, y - 1)
}


// WORK STARTS HERE

fn generate_two_primes(mut rng: &mut rand::prelude::ThreadRng) -> (u32, u32) {
    let mut p = 0;
    let mut q = 0;
    // Generally this loop should not execute more than once, but on the
    // off chance that we generate the same prime twice, we loop until
    // they are distinct.
    loop {
        p = get_random_prime(&mut rng);
        q = get_random_prime(&mut rng);
        if p != q {
            break;
        }
    }

    (p, q)
}


fn mod_inv(a: isize, module: isize) -> isize {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
}

// Modular multiplicative inverse code based on Rosetta Code's MMI code:
// https://rosettacode.org/wiki/Modular_inverse#Rust

fn mmi(a_unsigned: u32, m_unsigned: u32) -> u32 {

    let a: i64 = a_unsigned as i64;
    let m: i64 = m_unsigned as i64;
    
    let mut mn = (m, a);
    let mut xy = (0, 1);
    
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    
    while xy.0 < 0 {
        xy.0 += m;
    }

    if xy.0 > 0 {
        return xy.0 as u32;
    } else {
        panic!("Received negative inverse");
    }
}

fn choose_private_exponent(c: u32, mut rng: &mut rand::prelude::ThreadRng) -> u32 {

    let mut p = 0;
    
    loop {
        p = rng.gen_range(2, c);
        if is_coprime(p, c) {
            break;
        }
    }

    p

}

fn compute_public_exponent(e: u32, n: u32) -> u32 {
    mmi(e, n)
}

fn generate_key_pair(mut rng: &mut rand::prelude::ThreadRng) -> (u32, u32, u32) {
    // TODO
    // Step 1: Choose two distinct prime numbers, p and q
    let (p, q) = generate_two_primes(&mut rng);

    // Step 2: Compute m = p * q (will be the modulus)
    let m = p * q;

    // Step 3: Compute n = Carmichael's totient function of p, q
    //         Carmichael's Totient is simply lcm(p - 1, q - 1)
    let n = carmichael_totient(p, q);
    
    // Step 4: Choose some e which is coprime to n and 1 < e < n
    let e = choose_private_exponent(n, &mut rng);
    
    // Step 5: Compute the modular multiplicative inverse for d
    let d = compute_public_exponent(e, n);

    // Step 6: Perform a sanity check before returning.
    //         Verify that d * e = 1 modulo n.
    //         If it does not, panic!
    // if (d * e) % n != 1 {
    //     panic!("Error: (d * e) % n != 1");
    // }
    
    // Return a three-tuple with the following elements:
    // 1. Modulus (m)
    // 2. Private Exponent (d)
    // 3. Public Exponent (e)
    (m, d, e)
}

// fn hash(msg: String) -> u32 {

// }
    

fn sign_message(msg: String, priv_key_mod: u32, priv_key_exp: u32) -> u32 {
    // TODO

    // Step 1: Produce a hash value of the message.
// create a Sha256 object
// let mut hasher = Sha256::new();

// // write input message
// hasher.input_str("hello world");

// // read hash digest
// let hex = hasher.result_str();
    

    // Step 2: Raise it to the power of d, modulo n.

    1
}

fn verify_signature(msg: String, sig: String, pub_key_mod: u32, pub_key_exp: u32) -> bool {

    // Step 1: Get the hash value of the message.

    // Step 2: Raise it to the power of e modulo n.

    // Step 3: Compare the computed value in step 2 to the original hash
    //         calculated in Step 1.  If they match, the signature is valid.
    //         If not, it is invalid.
    false
}

/// Simple function to tell the user about appropriate usage and exit with exit code 1.
fn print_usage_and_exit() {
    println!("Usage:");
    println!("generate - generates a public/private keypair");
    println!("sign <msg> <priv_key_mod> <priv_key_exp>- signs a message with private key");
    println!("verify <msg> <signature> <pub_key_mod> <pub_key_exp> - verifies a message");
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
    } else if args.len() > 6 {
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
            if args.len() != 5 {
                return Err("sign requires three arguments".to_string());
            } else {
                return Ok(Function::Sign);
            }

        },
        "verify" => {
            if args.len() != 6 {
                return Err("verify requires four arguments".to_string());
            } else {
                return Ok(Function::Verify)                
            }

        },
        _ => {
            return Err("Unrecognized first argument".to_string());
        },
    }

}

fn print_keys(n: u32, d: u32, e: u32) {
    println!("Private key: {}, {}", n, d);
    println!("Public key: {}, {}", n, e);
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
                    let (n, d, e) = generate_key_pair(&mut rng);
                    print_keys(n, d, e);
                },
                Function::Sign => {
                    // sign_message(args[2], args[3], args[4]);
                },
                Function::Verify => {
                    // verify_signature(args[2], args[3], args[4], args[5]);
                },
            }
            // let a = rand::random::<u32>();
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

    // ****************************************************************
    // is_prime(n) function
    // ****************************************************************

    
    #[test]
    fn test_5_is_prime() {
        assert!(is_prime(5), "5 should be prime");
    }

    #[test]
    fn test_6_is_not_prime() {
        assert!(!is_prime(6), "6 should not be prime");
    }

    #[test]
    fn test_1000_is_not_prime() {
        assert!(!is_prime(1000), "1000 should not be prime");
    }
    
    #[test]
    fn test_1223_is_prime() {
        assert!(is_prime(1223), "1223 should be prime");
    }

    
    // ****************************************************************
    // get_random_prime() function
    // ****************************************************************

    #[test]
    fn test_gets_random_prime() {
        let mut rng = rand::thread_rng();
        let p = get_random_prime(&mut rng);
        assert!(is_prime(p));
    }

    // ****************************************************************
    // coprimes() function
    // ****************************************************************

    #[test]
    fn test_8_5_are_coprime() {
        assert!(is_coprime(8, 5), "8, 5 should be coprime");
    }

    #[test]
    fn test_8_6_are_not_coprime() {
        assert!(!is_coprime(8, 6), "8, 6 should not be coprime");
    }

    
}


