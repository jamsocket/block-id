use crate::{
    add_mod::{add_mod, sub_mod},
    transform::InvertableTransform,
};

pub struct Cascade {
    radix: u8,
}

impl Cascade {
    pub fn new(radix: u8) -> Self {
        Cascade { radix }
    }
}

impl InvertableTransform for Cascade {
    type Input = Vec<u8>;

    type Output = Vec<u8>;

    fn forward(&self, mut input: Vec<u8>) -> Vec<u8> {
        let mut last = 0;

        for v in input.iter_mut() {
            *v = add_mod(*v, last, self.radix);
            last = *v;
        }

        input
    }

    fn backward(&self, mut output: Vec<u8>) -> Vec<u8> {
        let mut last = 0;

        for v in output.iter_mut() {
            let tmp = *v;
            *v = sub_mod(*v, last, self.radix);
            last = tmp;
        }

        output
    }
}

#[cfg(test)]
mod test {
    use crate::transform::{test::round_trip, InvertableTransform};

    use super::Cascade;

    #[test]
    fn test_cascade() {
        {
            let cascade = Cascade::new(26);
            let input = vec![16, 16, 25];
            let expected = vec![16, 6, 5];

            assert_eq!(expected, cascade.forward(input));
        }

        {
            let cascade = Cascade::new(123);
            let input = vec![5, 9, 120, 5, 7, 15];
            let expected = vec![5, 14, 11, 16, 23, 38];

            assert_eq!(expected, cascade.forward(input));
        }
    }

    #[test]
    fn test_inverse() {
        let cascade = Cascade::new(123);
        let input = vec![5, 9, 120, 5, 7, 15];
        let expected = vec![5, 14, 11, 16, 23, 38];

        assert_eq!(input, cascade.backward(expected));
    }

    #[test]
    fn test_round_trip() {
        let cascade = Cascade::new(26);
        let input = vec![16, 16, 25];

        round_trip(&cascade, input);
    }
}
