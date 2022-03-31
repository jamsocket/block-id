use block_id::IdPermuter;

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let permuter = IdPermuter::new(&alphabet, 19, 6);
    let mut i: u64 = 1;

    loop {
        let code = permuter.encode_string(i);
        println!("{}: {}", i, code);

        let result = permuter.decode_string(&code);
        assert_eq!(i, result);

        i += 1;
    }
}
