use crate::{permutation::Permutation, transform::InvertableTransform};

#[derive(Clone)]
pub struct Permute {
    permutation: Permutation,
}

impl Permute {
    pub fn new(permutation: Permutation) -> Self {
        Self { permutation }
    }

    pub fn new_from_seed(length: u8, seed: u128) -> Self {
        let perm = Permutation::new_from_seed(length, seed);
        Self::new(perm)
    }
}

impl InvertableTransform for Permute {
    type Input = Vec<u8>;
    type Output = Vec<u8>;

    fn forward(&self, mut input: Vec<u8>) -> Option<Vec<u8>> {
        for elem in input.iter_mut() {
            *elem = self.permutation.forward(*elem)?;
        }
        Some(input)
    }

    fn backward(&self, mut output: Vec<u8>) -> Option<Vec<u8>> {
        for elem in output.iter_mut() {
            *elem = self.permutation.backward(*elem)?;
        }
        Some(output)
    }
}

#[cfg(test)]
mod test {
    use crate::transform::test::round_trip;

    use super::*;

    #[test]
    fn test_random() {
        let perm = Permute::new_from_seed(26, 118);

        let input = vec![24, 14];

        round_trip(&perm, input);
    }
}
