use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::transform::InvertableTransform;

#[derive(Clone)]
pub struct Permutation {
    forward: Vec<u8>,
    inverse: Vec<u8>,
}

impl Permutation {
    pub fn new(forward: Vec<u8>) -> Self {
        let size = forward.len() as u8;
        let mut inverse = vec![u8::MAX; forward.len()];
        for (i, v) in forward.iter().enumerate() {
            assert!(
                *v < size,
                "Elements in permutation must not exceed its length"
            );
            assert!(
                inverse[*v as usize] == u8::MAX,
                "Every value in permutation must be unique."
            );

            inverse[*v as usize] = i as u8;
        }

        Permutation { forward, inverse }
    }

    pub fn new_from_seed(length: u8, seed: u128) -> Self {
        let mut rng = Pcg64Mcg::new(seed);
        let perm = generate_random_permutation(&mut rng, length);

        Self::new(perm)
    }
}

impl InvertableTransform for Permutation {
    type Input = u8;
    type Output = u8;

    fn forward(&self, v: u8) -> Option<u8> {
        self.forward.get(v as usize).copied()
    }

    fn backward(&self, v: u8) -> Option<u8> {
        self.inverse.get(v as usize).copied()
    }
}

fn generate_random_permutation<T: Rng>(rng: &mut T, length: u8) -> Vec<u8> {
    let mut result: Vec<u8> = (0..length).collect();

    for i in 0..length {
        let swap_pos = rng.gen_range(i..length);
        result.swap(i as usize, swap_pos as usize);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        let perm = Permutation::new(vec![3, 1, 0, 2]);

        assert_eq!(3, perm.forward(0).unwrap());
        assert_eq!(1, perm.forward(1).unwrap());
        assert_eq!(0, perm.forward(2).unwrap());
        assert_eq!(2, perm.forward(3).unwrap());

        assert_eq!(1, perm.backward(1).unwrap());
        assert_eq!(0, perm.backward(3).unwrap());
        assert_eq!(2, perm.backward(0).unwrap());
        assert_eq!(3, perm.backward(2).unwrap());
    }

    #[test]
    #[should_panic(expected = "Every value in permutation must be unique")]
    fn test_permutation_duplicate_error() {
        Permutation::new(vec![3, 0, 1, 3]);
    }

    #[test]
    #[should_panic(expected = "Elements in permutation must not exceed its length")]
    fn test_permutation_bound_error() {
        Permutation::new(vec![3, 1]);
    }

    #[test]
    fn test_random_permutation() {
        for seed in 0..100 {
            for length in 1..30 {
                Permutation::new_from_seed(seed, length);
            }
        }
    }
}
