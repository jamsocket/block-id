use std::{collections::HashMap, hash::Hash};

use crate::transform::InvertableTransform;

pub struct Alphabet<T: Copy + Hash + Eq> {
    alphabet: Vec<T>,
    inv_index: HashMap<T, u8>,
}

impl<T: Copy + Hash + Eq> Alphabet<T> {
    pub fn new(alphabet: &[T]) -> Self {
        let alphabet: Vec<T> = alphabet.to_vec();
        let inv_index: HashMap<T, u8> = alphabet
            .iter()
            .enumerate()
            .map(|(k, v)| (*v, k as u8))
            .collect();
        assert_eq!(
            alphabet.len(),
            inv_index.len(),
            "Alphabet contained duplicate value(s)."
        );

        Self {
            alphabet,
            inv_index,
        }
    }

    pub fn len(&self) -> u8 {
        self.alphabet.len() as u8
    }
}

impl Alphabet<char> {
    pub fn alphanumeric() -> Alphabet<char> {
        let alpha: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        Self::new(&alpha)
    }

    pub fn lowercase_alphanumeric() -> Alphabet<char> {
        let alpha: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
        Self::new(&alpha)
    }

    pub fn lowercase_alpha() -> Alphabet<char> {
        let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        Self::new(&alpha)
    }
}

impl<T: Copy + Hash + Eq> InvertableTransform for Alphabet<T> {
    type Input = u8;

    type Output = T;

    fn forward(&self, index: u8) -> T {
        self.alphabet[index as usize]
    }

    fn backward(&self, value: T) -> u8 {
        self.inv_index[&value]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Alphabet contained duplicate value(s).")]
    fn test_duplicate() {
        Alphabet::new(&['b', 'l', 'a', 'h', 'b', 'c']);
    }

    #[test]
    fn test_forward() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let alpha = Alphabet::new(&chars);

        assert_eq!(26, alpha.len());

        assert_eq!('a', alpha.forward(0));
        assert_eq!('z', alpha.forward(25));
    }

    #[test]
    fn test_backward() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let alpha = Alphabet::new(&chars);

        assert_eq!(0, alpha.backward('a'));
        assert_eq!(25, alpha.backward('z'));
    }

    #[test]
    fn test_round_trip() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();
        let alpha = Alphabet::new(&chars);

        assert_eq!(52, alpha.len());

        for i in 0..chars.len() as u8 {
            let c = alpha.forward(i);
            let v = alpha.backward(c);

            assert_eq!(i, v);
        }
    }
}
