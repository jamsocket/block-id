pub trait InvertableTransform {
    type Input;
    type Output;

    fn forward(&self, input: Self::Input) -> Option<Self::Output>;

    fn backward(&self, output: Self::Output) -> Option<Self::Input>;
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::fmt::Debug;

    pub fn round_trip<T: InvertableTransform>(transform: &T, value: T::Input)
    where
        T::Input: PartialEq + Debug + Clone,
    {
        let output = transform.forward(value.clone()).unwrap();
        let result = transform.backward(output).unwrap();

        assert_eq!(result, value, "Input was not the same after a round trip.");
    }
}
