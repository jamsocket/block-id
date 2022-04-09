#![doc = include_str!("../README.md")]

pub use alphabet::Alphabet;
use base::BaseConversion;
use cascade::Cascade;
use permute::Permute;
use rotate::Rotate;
use std::{fmt::Debug, hash::Hash};
use transform::InvertableTransform;

mod add_mod;
mod alphabet;
mod base;
mod cascade;
mod permutation;
mod permute;
mod rotate;
mod transform;

/// Represents a specific, deterministic two-way mapping between `u64` values and opaque IDs.
///
/// For `BlockId<char>`, additional functionality is provided for mapping between `u64`s and
/// `String`s.
#[derive(Clone)]
pub struct BlockId<T: Copy + Hash + Eq> {
    alphabet: Alphabet<T>,
    base_convert: BaseConversion,
    cascade: Cascade,
    rotate: Rotate<u8>,
    permute: Permute,
}

impl<T: Copy + Hash + Eq> Debug for BlockId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockId with alphabet size {}", self.alphabet.len())
    }
}

impl<T: Copy + Hash + Eq> BlockId<T> {
    /// Construct a block ID mapping. Mappings are deterministic based on the three parameters
    /// passed at construction: the alphabet, seed, and minimum length.
    pub fn new(alphabet: Alphabet<T>, seed: u128, min_length: u8) -> Self {
        let base = alphabet.len();
        let base_convert = BaseConversion::new_with_min_length(base, min_length);
        let cascade = Cascade::new(base);
        let rotate = Rotate::new();
        let permute = Permute::new_from_seed(base, seed);

        BlockId {
            alphabet,
            base_convert,
            cascade,
            rotate,
            permute,
        }
    }

    /// Encode a given `u64` into an opaque `Vec<T>`.
    #[inline]
    pub fn encode(&self, v: u64) -> Vec<T> {
        self.forward(v)
    }

    /// Decode an opaque `Vec<T>` from a given `u64`.
    #[inline]
    pub fn decode(&self, v: Vec<T>) -> u64 {
        self.backward(v)
    }
}

impl<T: Copy + Hash + Eq> InvertableTransform for BlockId<T> {
    type Input = u64;
    type Output = Vec<T>;

    fn forward(&self, v: u64) -> Vec<T> {
        let mut v = self.base_convert.forward(v);

        for _ in 0..v.len() {
            v = self.permute.forward(v);
            v = self.cascade.forward(v);
            v = self.rotate.forward(v);
        }

        v.iter().map(|d| self.alphabet.forward(*d)).collect()
    }

    fn backward(&self, v: Vec<T>) -> u64 {
        let mut v: Vec<u8> = v.iter().map(|d| self.alphabet.backward(*d)).collect();

        for _ in 0..v.len() {
            v = self.rotate.backward(v);
            v = self.cascade.backward(v);
            v = self.permute.backward(v);
        }

        self.base_convert.backward(v)
    }
}

/// For the special case of `BlockId<char>`, helper encoders/decoders that use strings
/// rather than vectors are provided.
impl BlockId<char> {
    /// Encode a `u64` into an opaque string.
    pub fn encode_string(&self, v: u64) -> String {
        self.forward(v).into_iter().collect()
    }

    /// Decode a `u64` from an opaque string.
    pub fn decode_string(&self, v: &str) -> u64 {
        self.backward(v.chars().collect())
    }
}

#[cfg(test)]
mod test {
    use crate::{transform::test::round_trip, Alphabet, BlockId};

    #[test]
    fn test_round_trip() {
        let permuter = BlockId::new(Alphabet::lowercase_alpha(), 118, 4);

        for i in 600..800 {
            round_trip(&permuter, i);
        }
    }

    #[test]
    fn test_debug() {
        let block1 = BlockId::new(Alphabet::lowercase_alpha(), 118, 4);
        assert_eq!("BlockId with alphabet size 26", format!("{:?}", block1));

        let block2 = BlockId::new(Alphabet::alphanumeric(), 118, 4);
        assert_eq!("BlockId with alphabet size 62", format!("{:?}", block2));
    }
}
