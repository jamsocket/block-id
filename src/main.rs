use block_id::{IdPermuter, InvertableTransform};

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let permuter = IdPermuter::new(&alphabet, 144);
    let mut i: u64 = 1;

    loop {
        let code = permuter.forward(i);
        let code_str: String = code.iter().collect();
        println!("{}: {}", i, code_str);
        
        let result = permuter.backward(code);
        assert_eq!(i, result);
        
        i += 1;
    }
}