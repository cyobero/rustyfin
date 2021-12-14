use num::Num;

pub trait MovingAverage<Output = Vec<f64>> {
    /// Calculate the n-period simple moving average.
    /// Example:
    ///     let nums = vec![5., 7., 6., 5., 5.5, 4.5];
    ///     let sma = nums.sma(2);
    ///     assert_eq!(sma, vec![6.0, 7.5, 7.0, 5.5, 5.25]);
    fn sma(&self, periods: usize) -> Output;

    /// Calculate the n-period exponentially-weighted moving average.
    /// Example:
    ///     let nums = vec![2., 4., 6., 8., 12.];
    ///     let ema = nums.ema(2);
    ///     assert_eq!(ema, vec![3.0, 6.333333333333333, 10.11111111111111]);
    fn ema(&self, periods: usize) -> Output;
}

pub trait Volatility<T: Copy + PartialOrd> {
    /// Calculate the difference between the max value and min value of `[T]`
    /// Example:
    ///     let nums = [10, 5, 2, 1, 3, 7];
    ///     let range = nums.range_volatility();
    ///     assert_eq!(range, 9);
    fn range_volatility(&self) -> T;
}

impl<T: Copy + Into<f64>> MovingAverage for [T] {
    fn sma(&self, periods: usize) -> Vec<f64> {
        let mut sum = 0f64;
        let mut ma = Vec::<f64>::with_capacity(self.len() - periods);
        for i in 0..self.len() {
            if i >= periods {
                if i >= periods {
                    ma.push(sum / periods as f64);
                    sum -= self[i - periods].into();
                }
            }
            sum += self[i].into();
        }
        ma
    }

    fn ema(&self, periods: usize) -> Vec<f64> {
        let mut sum = 0f64;
        let mut ma = Vec::<f64>::with_capacity(self.len() - periods);
        let multiplier = 2. / (periods + 1) as f64;
        for i in 0..self.len() {
            if i == periods {
                ma.push(sum / periods as f64);
                sum -= self[i - periods].into();
            } else if i > periods {
                let val = (1. - multiplier) * ma.last().unwrap() + multiplier * &self[i].into();
                ma.push(val);
            }
            sum += self[i].into();
        }
        ma
    }
}

impl<T: Num + PartialOrd + Copy + From<i32>> Volatility<T> for [T] {
    fn range_volatility(&self) -> T {
        let mut min = &self[0];
        let mut max = min;
        if self.len() <= 1 {
            return T::from(0);
        } else {
            for n in self {
                if n < min {
                    min = n;
                } else if n > max {
                    max = n;
                }
            }
        }
        *max - *min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma_calculated_correctly() {
        let nums = vec![5., 7., 8., 6., 5., 5.5, 4.5];
        let sma = nums.sma(2);
        assert_eq!(sma, vec![6.0, 7.5, 7.0, 5.5, 5.25]);
    }

    #[test]
    fn ema_calculated_correctly() {
        let nums = vec![2., 4., 6., 8., 12.];
        let ema = nums.ema(2);
        assert_eq!(ema, vec![3.0, 6.333333333333333, 10.11111111111111]);
    }

    #[test]
    fn range_volatility_calculated_correctly() {
        let nums = [10, 5, 2, 1, 3, 7];
        let range = nums.range_volatility();
        assert_eq!(range, 9);
    }

    #[test]
    fn range_volatility_returns_zero_for_one_element_vec() {
        let nums = vec![5.];
        let range = nums.range_volatility();
        assert_eq!(range, 0f64);
    }
}
