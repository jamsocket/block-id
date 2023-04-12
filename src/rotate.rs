use std::marker::PhantomData;

use crate::transform::InvertableTransform;

#[derive(Clone)]
pub struct Rotate<T> {
    _ph: PhantomData<T>,
}

impl<T> Rotate<T> {
    pub fn new() -> Self {
        Rotate {
            _ph: PhantomData::default(),
        }
    }
}

impl<T> InvertableTransform for Rotate<T> {
    type Input = Vec<T>;
    type Output = Vec<T>;

    fn forward(&self, mut value: Vec<T>) -> Option<Vec<T>> {
        value.rotate_left(1);
        Some(value)
    }

    fn backward(&self, mut value: Vec<T>) -> Option<Vec<T>> {
        value.rotate_right(1);
        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        let rot: Rotate<u8> = Rotate::new();
        let before = vec![4, 5, 6, 7, 8];
        let after = vec![5, 6, 7, 8, 4];

        assert_eq!(after.clone(), rot.forward(before.clone()).unwrap());
        assert_eq!(before, rot.backward(after).unwrap());
    }
}
