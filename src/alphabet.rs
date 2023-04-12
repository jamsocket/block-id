use std::{collections::HashMap, hash::Hash};

use crate::transform::InvertableTransform;

/// Represents a set of characters that are valid in a [`crate::BlockId`].
///
/// An alphabet is generic over the type of data used to represent a "character",
/// but `Alphabet<char>` is used when generating string codes (otherwise, `BlockId`
/// will generate a `Vec<T>`).
///
/// Several built-in alphabets are provided.
///
/// Examples:
/// ```rust
/// # use block_id::Alphabet;
/// # fn main() {
/// let alpha1: Alphabet<char> = Alphabet::alphanumeric();
/// let hexchars: Vec<char> = "0123456789abcdef".chars().collect();
/// let alpha2: Alphabet<char> = Alphabet::new(&hexchars);
/// let alpha3: Alphabet<u32> = Alphabet::new(&[1234, 5678, 2345]);
/// # }
/// ```
#[derive(Clone)]
pub struct Alphabet<T: Copy + Hash + Eq> {
    alphabet: Vec<T>,
    inv_index: HashMap<T, u8>,
}

impl<T: Copy + Hash + Eq> Alphabet<T> {
    /// Construct an alphabet from a list of values.
    ///
    /// Panics if the alphabet contains duplicates, or contains more than 256 elements.
    pub fn new(alphabet: &[T]) -> Self {
        let alphabet: Vec<T> = alphabet.to_vec();

        assert!(
            alphabet.len() <= u8::MAX as usize,
            "Alphabet is too long (up to 256 elements supported)."
        );

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

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u8 {
        self.alphabet.len() as u8
    }
}

impl Alphabet<char> {
    /// Returns an alphabet with lower- and upper-case letters, and numeral digits.
    pub fn alphanumeric() -> Alphabet<char> {
        let alpha: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();
        Self::new(&alpha)
    }

    /// Returns an alpahbet with lower-case letters and numeral digits.
    pub fn lowercase_alphanumeric() -> Alphabet<char> {
        let alpha: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
        Self::new(&alpha)
    }

    /// Returns an alphabet with lowercase letters only.
    pub fn lowercase_alpha() -> Alphabet<char> {
        let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        Self::new(&alpha)
    }
}

impl<T: Copy + Hash + Eq> InvertableTransform for Alphabet<T> {
    type Input = u8;

    type Output = T;

    fn forward(&self, index: u8) -> Option<T> {
        self.alphabet.get(index as usize).copied()
    }

    fn backward(&self, value: T) -> Option<u8> {
        self.inv_index.get(&value).copied()
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
    #[should_panic(expected = "Alphabet is too long (up to 256 elements supported).")]
    fn test_too_long() {
        let chars: Vec<u16> = (0..30000).collect();
        Alphabet::new(&chars);
    }

    #[test]
    fn test_invalid_character() {
        let chars = Alphabet::lowercase_alpha();
        assert_eq!(None, chars.backward('!'));

        assert_eq!(None, chars.forward(26));
    }

    #[test]
    fn test_forward() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let alpha = Alphabet::new(&chars);

        assert_eq!(26, alpha.len());

        assert_eq!('a', alpha.forward(0).unwrap());
        assert_eq!('z', alpha.forward(25).unwrap());
    }

    #[test]
    fn test_backward() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let alpha = Alphabet::new(&chars);

        assert_eq!(0, alpha.backward('a').unwrap());
        assert_eq!(25, alpha.backward('z').unwrap());
    }

    #[test]
    fn test_round_trip() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();
        let alpha = Alphabet::new(&chars);

        assert_eq!(52, alpha.len());

        for i in 0..chars.len() as u8 {
            let c = alpha.forward(i).unwrap();
            let v = alpha.backward(c).unwrap();

            assert_eq!(i, v);
        }
    }
}
