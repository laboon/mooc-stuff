
use std::env;

const BLOCK_SIZE: usize = 8;
const INITIALIZATION_VECTOR: u64 = 0x0123_4567_89AB_CDEF;

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
    if rem == 0 && data.len() > 0 {
        // do nothing, no padding necessary
        data
    } else {
        let mut to_return = data;
        let n = BLOCK_SIZE - rem;
        for _j in 0..n {
            to_return.push(0);
        }
        to_return
    }
}

fn twiddle(arr: &mut [u8; BLOCK_SIZE]) {

    for j in 0..BLOCK_SIZE {
        arr[j % BLOCK_SIZE] ^=
            arr[(j + 1) % BLOCK_SIZE] << ((j + 7) % BLOCK_SIZE)
            ^ arr[(j + 2) % BLOCK_SIZE] << ((j + 6) % BLOCK_SIZE)
            ^ arr[(j + 3) % BLOCK_SIZE] << ((j + 5) % BLOCK_SIZE)
            ^ arr[(j + 4) % BLOCK_SIZE] << ((j + 4) % BLOCK_SIZE)
            ^ arr[(j + 5) % BLOCK_SIZE] >> ((j + 3) % BLOCK_SIZE)
            ^ arr[(j + 6) % BLOCK_SIZE] >> ((j + 2) % BLOCK_SIZE)
            ^ arr[(j + 7) % BLOCK_SIZE] >> ((j + 1) % BLOCK_SIZE);
    }
}


fn transform(cv: u64, arr: [u8; BLOCK_SIZE]) -> u64 {
    let mut to_return: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    let cv_arr: [u8; BLOCK_SIZE] = cv.to_le_bytes();

    for j in 0..BLOCK_SIZE {
        to_return[j] = arr[j] ^ cv_arr[j];
    }

    for j in 0..1024 {
        twiddle(&mut to_return);
    }

    u64::from_le_bytes(to_return)

}

fn compress(cv: u64, data: Vec<u8>) -> u64 {
    let mut new_data = data;
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
    to_finalize ^ 0xFFFF_FFFF_FFFF_FFFF
}

fn bill_hash(h: Vec<u8>) -> u64 {
    let mut cv: u64 = INITIALIZATION_VECTOR;
    let blocks = split(h);
    for block in blocks {
        cv = compress(cv, block);
    }

    finalize(cv)
}



fn main() {

    let hash_val = bill_hash(get_to_hash());
    println!("Hash value: {:#016x}", hash_val);

}

#[cfg(test)]
mod tests {
    use super::*;

    // strengthen

    #[test]
    fn test_strengthen_empty_arr() {
        let to_test = Vec::new();
        let expected = [0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(strengthen(to_test), expected);
    }

    #[test]
    fn test_strengthen_one_elem_arr() {
        let to_test = vec![1];
        let expected = [1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(strengthen(to_test), expected);
    }

    #[test]
    fn test_strengthen_four_elem_arr() {
        let to_test = vec![0, 1, 2, 3];
        let expected = [0, 1, 2, 3, 0, 0, 0, 0];
        assert_eq!(strengthen(to_test), expected);
    }

    #[test]
    fn test_strengthen_eight_elem_arr() {
        let to_test = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let expected = [0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(strengthen(to_test), expected);
    }

    // split

    #[test]
    fn test_split_empty_arr() {
        let to_test = Vec::new();
        let expected = [[0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(split(to_test), expected);
    }

    #[test]
    fn test_split_single_elem() {
        let to_test = vec![1];
        let expected = [[1, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(split(to_test), expected);
    }

    #[test]
    fn test_split_same_as_block_size() {
        let to_test = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let expected = [[0, 1, 2, 3, 4, 5, 6, 7]];
        assert_eq!(split(to_test), expected);
    }

    #[test]
    fn test_split_one_more_than_block_size() {
        let to_test = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = [[0, 1, 2, 3, 4, 5, 6, 7],
                        [8, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(split(to_test), expected);
    }

    #[test]
    fn test_split_several_blocks_no_padding() {
        let to_test = vec![0, 1, 2, 3, 4, 5, 6, 7, 8,
        9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3];
        let expected = [[0, 1, 2, 3, 4, 5, 6, 7],
                        [8, 9, 0, 1, 2, 3, 4, 5],
                        [6, 7, 8, 9, 0, 1, 2, 3]];
        assert_eq!(split(to_test), expected);
    }


    #[test]
    fn test_split_several_blocks_with_padding() {
        let to_test = vec![0, 1, 2, 3, 4, 5, 6, 7, 8,
        9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected = [[0, 1, 2, 3, 4, 5, 6, 7],
                        [8, 9, 0, 1, 2, 3, 4, 5],
                        [6, 7, 8, 9, 0, 0, 0, 0]];
        assert_eq!(split(to_test), expected);
    }

    // transform

    #[test]
    fn test_transform_iv_0() {
        let cv = INITIALIZATION_VECTOR;
        let to_test = [0, 1, 2, 3, 4, 5, 6, 7];
        // assert_eq!(transform(cv, to_test), 15536890553721539187);

    }


    // compress

    // finalize

    #[test]
    fn test_finalize_0() {
        assert_eq!(finalize(0), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_finalize_maxint() {
        assert_eq!(finalize(0xFFFFFFFFFFFFFFFF), 0);
    }

    #[test]
    fn test_finalize_mixed() {
        assert_eq!(finalize(0xF0F0F0F0F0F0F0F0), 0x0F0F0F0F0F0F0F0F);
    }

    #[test]
    fn test_finalize_some_value() {
        assert_eq!(finalize(3987120094), 18446744069722431521);
    }

    // bill_hash

}
