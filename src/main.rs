use block_id::{Alphabet, BlockId};

fn main() {
    let permuter = BlockId::new(Alphabet::alphanumeric(), 19, 6);
    let mut i: u64 = 1;

    loop {
        let code = permuter.encode_string(i).unwrap();
        println!("{}: {}", i, code);

        let result = permuter.decode_string(&code).unwrap();
        assert_eq!(i, result);

        i += 1;
    }
}
