#![doc = include_str!("../README.md")]

pub use alphabet::Alphabet;
use base::BaseConversion;
use cascade::Cascade;
use permute::Permute;
use rotate::Rotate;
use std::hash::Hash;
use transform::InvertableTransform;

mod add_mod;
mod alphabet;
mod base;
mod cascade;
mod permutation;
mod permute;
mod rotate;
mod transform;

pub struct BlockId<T: Copy + Hash + Eq> {
    alphabet: Alphabet<T>,
    base_convert: BaseConversion,
    cascade: Cascade,
    rotate: Rotate<u8>,
    permute: Permute,
}

impl<T: Copy + Hash + Eq> BlockId<T> {
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

    #[inline]
    pub fn forward(&self, v: u64) -> Vec<T> {
        InvertableTransform::forward(self, v)
    }

    #[inline]
    pub fn backward(&self, v: Vec<T>) -> u64 {
        InvertableTransform::backward(self, v)
    }
}

impl<T: Copy + Hash + Eq> InvertableTransform for BlockId<T> {
    type Input = u64;
    type Output = Vec<T>;

    fn forward(&self, v: u64) -> Vec<T> {
        let mut v = self.base_convert.forward(v + 1);

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

        self.base_convert.backward(v) - 1
    }
}

impl BlockId<char> {
    pub fn encode_string(&self, v: u64) -> String {
        self.forward(v).into_iter().collect()
    }

    pub fn decode_string(&self, v: &str) -> u64 {
        self.backward(v.chars().collect())
    }
}

#[cfg(test)]
mod test {
    use crate::{transform::test::round_trip, BlockId, Alphabet};

    #[test]
    fn test_round_trip() {
        let permuter = BlockId::new(Alphabet::lowercase_alpha(), 118, 4);

        for i in 600..800 {
            round_trip(&permuter, i);
        }
    }
}
