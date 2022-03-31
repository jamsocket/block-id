use crate::transform::InvertableTransform;

pub struct BaseConversion {
    radix: u8,
}

impl BaseConversion {
    pub fn new(radix: u8) -> Self {
        BaseConversion { radix }
    }
}

impl InvertableTransform for BaseConversion {
    type Input = u64;
    type Output = Vec<u8>;

    fn forward(&self, mut input: u64) -> Vec<u8> {
        let base = self.radix;
        let mut result = Vec::new();

        while input > 0 {
            result.push((input % base as u64) as u8);
            input /= base as u64;
        }

        result.reverse();

        result
    }

    fn backward(&self, data: Vec<u8>) -> u64 {
        let mut result: u64 = 0;
        let base = self.radix;

        for (i, b) in data.iter().enumerate() {
            if i > 0 {
                result *= base as u64;
            }
            result += *b as u64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::transform::test::round_trip;

    use super::*;

    #[test]
    fn test_base_convert() {
        assert_eq!(vec![5], BaseConversion::new(128).forward(5));

        assert_eq!(vec![1, 0, 1, 0], BaseConversion::new(2).forward(10));

        assert_eq!(vec![1, 0, 1], BaseConversion::new(3).forward(10));
        assert_eq!(vec![1, 0, 2], BaseConversion::new(3).forward(11));
        assert_eq!(vec![1, 1, 0], BaseConversion::new(3).forward(12));
        assert_eq!(vec![1, 0, 0, 0], BaseConversion::new(3).forward(27));
    }

    #[test]
    fn test_base_invert() {
        assert_eq!(5, BaseConversion::new(128).backward(vec![5]));

        assert_eq!(10, BaseConversion::new(2).backward(vec![1, 0, 1, 0]));

        assert_eq!(10, BaseConversion::new(3).backward(vec![1, 0, 1]));
        assert_eq!(11, BaseConversion::new(3).backward(vec![1, 0, 2]));
        assert_eq!(12, BaseConversion::new(3).backward(vec![1, 1, 0]));
        assert_eq!(27, BaseConversion::new(3).backward(vec![1, 0, 0, 0]));
    }

    #[test]
    fn test_grind() {
        for base in 2..125 {
            for num in 88..99 {
                round_trip(&BaseConversion::new(base), num);
            }
        }
    }
}
