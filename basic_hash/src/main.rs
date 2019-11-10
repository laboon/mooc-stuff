
use std::env;

const BLOCK_SIZE: usize = 8;
const INITIALIZATION_VECTOR: u64 = 0x0123456789ABCDEF;

fn print_usage_and_exit() {
    println!("Please enter one and only one argument");
    std::process::exit(1);
}

fn convert_string_to_u8s(s: String) -> Vec<u8> {
    s.as_bytes().to_vec()
}

fn get_to_hash() -> Vec<u8> {
    let mut args = Vec::new();
    for argument in env::args() {
        args.push(argument);
    }

    // ignore "0 arg", i.e. executable
    let args_len = args.len() - 1;

    if args_len != 1 {
        print_usage_and_exit();
    }

    convert_string_to_u8s(args[1].clone())

}

fn strengthen(data: Vec<u8>) -> Vec<u8> {
    let rem = data.len() % BLOCK_SIZE;
    if rem == 0 {
        // do nothing, no padding necessary
        return data;
    } else {
        let mut to_return = data.clone();
        let n = BLOCK_SIZE - rem;
        for _j in 0..n {
            to_return.push(0);
        }
        return to_return;
    }
}

fn transform(cv: u64, arr: [u8; BLOCK_SIZE]) -> u64 {
    let mut to_return: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    let cv_arr: [u8; BLOCK_SIZE] = cv.to_le_bytes();

    for j in 0..BLOCK_SIZE {
        to_return[j] = arr[j] ^ cv_arr[j];
    }

    for j in 0..1024 {
        to_return[j % BLOCK_SIZE] ^= to_return[(j + 1) % BLOCK_SIZE] << 1 ^ to_return[(j + 2) % BLOCK_SIZE] >> 1 ^ to_return[(j + 3) % BLOCK_SIZE] << 2 ^ to_return[(j + 4) % BLOCK_SIZE] >> 2;
    }

    u64::from_le_bytes(to_return)

}

fn compress(cv: u64, data: Vec<u8>) -> u64 {
    let mut new_data = data.clone();
    let mut a: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    for j in 0..BLOCK_SIZE {
        a[j] = new_data.pop().unwrap();
    }

    transform(cv, a)
}

fn split(data: Vec<u8>) -> Vec<Vec<u8>> {
    let to_split = strengthen(data);
    let mut to_return = Vec::new();
    let num_blocks = to_split.len() / BLOCK_SIZE;
    let mut counter = 0;
    for _j in 0..num_blocks {
        let mut new_block = Vec::new();
        for _k in 0..BLOCK_SIZE {
            new_block.push(to_split[counter]);
            counter += 1;
        }
        to_return.push(new_block);
    }
    to_return

}

fn finalize(to_finalize: u64) -> u64 {
    to_finalize ^ 0xFFFFFFFF
}

fn bill_hash(h: Vec<u8>) -> u64 {
    let mut cv: u64 = INITIALIZATION_VECTOR;
    let blocks = split(h);
    println!("cv = {} (IV)", cv);
    for block in blocks {
        cv = compress(cv, block);
        println!("cv = {}", cv);
    }

    finalize(cv)
}


fn main() {

    let to_hash = get_to_hash();

    println!("{:?}", to_hash);

    let hash_val = bill_hash(to_hash);

    println!("Hash value: {:#016x}", hash_val);

}
