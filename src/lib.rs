use core::ops::Rem;
use num_traits::Zero;

fn is_divisible<N, R>(dividend: N, divisor: N) -> bool
where
    N: Rem<Output = R>,
    R: Zero,
{
    (dividend % divisor).is_zero()
}

#[derive(Debug, Clone)]
pub struct Divisors<N, I> {
    dividend: N,
    divisors: I,
}

impl<N, I> Divisors<N, I> {
    fn new(dividend: N, divisors: I) -> Self {
        Self { dividend, divisors }
    }
}

impl<N, R, I> Iterator for Divisors<N, I>
where
    N: Rem<Output = R> + Copy,
    R: Zero,
    I: Iterator<Item = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.divisors.find(|d| is_divisible(self.dividend, *d))
    }
}

impl<N, R, I> DoubleEndedIterator for Divisors<N, I>
where
    N: Rem<Output = R> + Copy,
    R: Zero,
    I: Iterator<Item = N> + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.divisors.rfind(|d| is_divisible(self.dividend, *d))
    }
}

pub trait IntoDivisors<N, I> {
    fn divisors(self, dividend: N) -> Divisors<N, I>;
}

impl<N, I, T> IntoDivisors<N, I> for T
where
    T: IntoIterator<IntoIter = I>,
{
    fn divisors(self, dividend: N) -> Divisors<N, I> {
        Divisors::new(dividend, self.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_20() {
        let div: Vec<_> = (1..=20).divisors(20).collect();
        assert_eq!(&div, &[1, 2, 4, 5, 10, 20])
    }

    #[test]
    fn rdiv_20() {
        let div: Vec<_> = (1..=20).divisors(20).rev().collect();
        assert_eq!(&div, &[20, 10, 5, 4, 2, 1])
    }
}
