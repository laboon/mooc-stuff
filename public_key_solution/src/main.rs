extern crate num_bigint;

use num::bigint::BigInt;
use num::bigint::{ToBigInt, RandBigInt};
use num::FromPrimitive;

use rand::prelude::*;
use std::env;
use std::process::*;
use num::integer::*;
use modinverse::modinverse;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use modpow::*;
use num::bigint::*;

// Our two keys can not be higher than this value
// This makes cracking the code relatively simple, but frees us
// from having to use bigint everywhere for multiplication!
const MAX_KEY_VAL: usize = 65_536;

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

fn is_prime(n: BigInt) -> bool {
    if n <= 3.to_bigint().unwrap() {
        return n > 1.to_bigint().unwrap();
    } else if n % 2.to_bigint().unwrap() == 0.to_bigint().unwrap()
        || n % 3.to_bigint().unwrap() == 0.to_bigint().unwrap() {
            
        return false;
    }

    let mut i = 5.to_bigint().unwrap();

    while i * i <= n {
        if n % i == 0.to_bigint().unwrap()
            || n % (i + 2.to_bigint().unwrap()) == 0.to_bigint().unwrap() {
            return false;
        }
        i = i + 6.to_bigint().unwrap();
    }
    
    return true;
    
}

/// This function will return a random prime.
/// It does this by randomly generating an integer and testing if it's
/// prime.  There are definitely more efficient algorithms for this,
/// but this is meant to be as simple as possible.

fn get_random_prime(rng: &mut rand::prelude::ThreadRng) -> BigInt {

    let mut p;

    // Keep generating random numbers and putting them in `p` until
    // the generated number is found to be prime.
    // Note that Rust does not have a do...while equivalent, so this
    // break statement may seem strange if you are coming from a different
    // language.
    loop {
        
        p = rng.gen_bigint(MAX_KEY_VAL);

        if is_prime(p) {
            break;
        }

    }

    // Return the last generated number, which should be prime
    p

}

fn is_coprime(x: BigInt, y: BigInt) -> bool {
    num::integer::gcd(x, y) == 1.to_bigint().unwrap()
}

fn carmichael_totient(x: BigInt, y: BigInt) -> BigInt {
    num::integer::lcm(x - 1, y - 1)
}


// WORK STARTS HERE

fn generate_two_primes(mut rng: &mut rand::prelude::ThreadRng) -> (BigInt, BigInt) {
    let mut p;
    let mut q;
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


// Modular multiplicative inverse code based on Rosetta Code's MMI code:
// https://rosettacode.org/wiki/Modular_inverse#Rust

fn mmi(a: &BigInt, m: &BigInt) -> BigInt {

    let mut mn = (*m, *a);
    let mut xy = (0.to_bigint().unwrap(), 1.to_bigint().unwrap());
    
    while mn.1 != 0.to_bigint().unwrap() {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    
    while xy.0 < 0.to_bigint().unwrap() {
        xy.0 += m;
    }

    if xy.0 > 0.to_bigint().unwrap() {
        return xy.0;
    } else {
        panic!("Received negative inverse");
    }
}


fn choose_private_exponent(c: &BigInt, rng: &mut rand::prelude::ThreadRng) -> BigInt {

    let mut p;
    
    loop {
        p = rng.gen_bigint_range(&2.to_bigint().unwrap(), &c);
        if is_coprime(p, c) {
            break;
        }
    }

    p

}

fn compute_public_exponent(e: &BigInt, n: &BigInt) -> BigInt {
    mmi(e, n)
}

fn generate_key_pair(mut rng: &mut rand::prelude::ThreadRng) -> (BigInt, BigInt, BigInt) {
    // TODO
    // Step 1: Choose two distinct prime numbers, p and q
    let (p, q) = generate_two_primes(&mut rng);

    // Step 2: Compute m = p * q - this will be the modulus
    let m = p * q;

    // Step 3: Compute n = Carmichael's totient function of p, q
    //         Carmichael's Totient is simply lcm(p - 1, q - 1)
    let n = carmichael_totient(&p, &q);
    
    // Step 4: Choose some e which is coprime to n and 1 < e < n
    let e = choose_private_exponent(&n, &mut rng);
    
    // Step 5: Compute the modular multiplicative inverse for d
    let d = compute_public_exponent(&e, &n);

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

fn get_hash<T: Hash>(t: &T) -> BigInt {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let r = s.finish();
    BigInt::from_u64(r).unwrap()
}


fn sign_message(msg: String, priv_key_mod: BigInt, priv_key_exp: BigInt) -> BigInt {
    // TODO

    // Step 1: Produce a hash value of the message.
    let h = get_hash(&msg);
    
    // Step 2: Raise it to the power of d, modulo n.
    let r  = modpow(&h, &priv_key_exp, &priv_key_mod);

    r
    
}

fn verify_signature(msg: String, sig: BigInt, pub_key_mod: BigInt, pub_key_exp: BigInt) -> bool {

    // Step 1: Get the hash value of the message.
    let h = get_hash(&msg);
  
    // Step 2: Raise it to the power of e modulo n.
    let r  = modpow(&h, &pub_key_exp, &pub_key_mod);

    // Step 3: Compare the computed value in step 2 to the original hash
    //         calculated in Step 1.  If they match, the signature is valid.
    //         If not, it is invalid
    
    r == h
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

fn args_good(args: &Vec<String>) -> Result<Function, String> {

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

fn print_keys(n: BigInt, d: BigInt, e: BigInt) {
    println!("Private key: {}, {}", n, d);
    println!("Public key: {}, {}", n, e);
}


fn main() {
    
    let mut args = Vec::new();
    for argument in env::args() {
        args.push(argument);
    }

    let args_ok = args_good(&args);
    match args_ok {
        Ok(f) => {
            match f {
                Function::Generate => {
                    let mut rng = rand::thread_rng();
                    let (n, d, e) = generate_key_pair(&mut rng);
                    print_keys(n, d, e);
                },
                Function::Sign => {
                    let msg: String = args[2].clone();
                    let priv_key_mod = args[3].parse::<BigInt>().unwrap();
                    let priv_key_exp = args[4].parse::<BigInt>().unwrap();
                    let sig = sign_message(msg, priv_key_mod, priv_key_exp);
                    println!("Signature: {}", sig);
                },
                Function::Verify => {
                    let msg: String = args[2].clone();
                    let sig = args[3].parse::<BigInt>().unwrap();
                    let pub_key_mod = args[4].parse::<BigInt>().unwrap();
                    let pub_key_exp = args[5].parse::<BigInt>().unwrap();

                    let r = verify_signature(msg, sig, pub_key_mod, pub_key_exp);
                    match r {
                        true => { println!("Signature verified!"); }
                        false => { println!("SIGNATURE INVALID!"); }
                    }
                },
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            print_usage_and_exit();
        },
    }
}

// Sample values for your own testing
// Private key: 2567355503, 190339441 (mod, exp)
// Public key: 2567355503, 643681741 (mod, exp)

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
