use block_id::{Alphabet, BlockId};

fn main() {
    let permuter = BlockId::new(Alphabet::alphanumeric(), 19, 6);
    let mut i: u64 = 1;

    loop {
        let code = permuter.encode_string(i);
        println!("{}: {}", i, code);

        let result = permuter.decode_string(&code);
        assert_eq!(i, result);

        i += 1;
    }
}
