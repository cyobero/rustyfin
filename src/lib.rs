use self::errors::Error;
use num::Num;
use std::ops::{AddAssign, Sub};

pub mod errors;

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

pub trait CentralMoment<Output = f64>
where
    Output: Copy + Into<f64> + From<f64>,
{
    /// Calculate the mean of a series.
    /// Example:
    ///     let nums = vec![5, 4, 3, 4];
    ///     let avg = nums.mean();
    ///     assert_eq!(avg, 4.);
    fn mean(&self) -> Output;

    /// Calculate the population variance of a series.
    /// Example:
    ///     let nums = vec![10.5, 2.1, 1.9, 10.3, 9.8];
    ///     let var = nums.var();
    ///     assert_eq!(var, 2.242);
    fn var(&self) -> Output;

    /// Calculate the population standard deviation of a series.
    /// Example:
    ///     let nums = vec![5., 5., 10., 3.];
    ///     let var = nums.var();
    ///     assert_eq!(nums.std(), var.sqrt());
    fn std(&self) -> Output {
        let var: f64 = self.var().into();
        Output::from(var.sqrt())
    }
}

pub trait Covariance<Output = f64, E = Error> {
    /// Calculate the covariance between of series.
    fn covar(&self, cmp: &Self) -> Result<Output, E>;
}

impl<T: Copy + Into<f64>> Covariance for [T]
where
    Self: CentralMoment,
{
    fn covar(&self, cmp: &Self) -> Result<f64, Error> {
        match self.len() == cmp.len() {
            false => Err(Error::LengthMismatch),
            true => {
                let mut sum = 0f64;
                let (mean1, mean2) = (self.mean(), cmp.mean());
                for i in 0..self.len() {
                    sum += (self[i].into() - mean1) * (cmp[i].into() - mean2);
                }
                Ok(sum / (self.len() as f64 - 1.))
            }
        }
    }
}

impl<T: Copy + Into<f64>> CentralMoment for [T]
where
    T: Copy + Into<f64>,
{
    fn mean(&self) -> f64 {
        let mut sum = 0f64;
        for i in 0..self.len() {
            sum += self[i].into();
        }
        sum / self.len() as f64
    }

    fn var(&self) -> f64 {
        let mean: f64 = self.mean();
        let mut ss = 0f64;
        for n in self {
            let sq_diff = ((*n).into() - mean).powf(2.);
            ss += sq_diff;
        }

        ss / (self.len() as f64)
    }
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
        let numsf = vec![5., 7., 8., 6., 5., 5.5, 4.5];
        let sma = numsf.sma(2);
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
        let nums = vec![5];
        let range = nums.range_volatility();
        assert_eq!(range, 0);
    }

    #[test]
    fn mean_calculated_for_integers() {
        let nums = vec![5, 4, 3, 4];
        let avg = nums.mean();
        assert_eq!(avg, 4.);
    }

    #[test]
    fn mean_calculated_for_f32s() {
        let nums = vec![5., 4.5, 3.5, 3.];
        let avg = nums.mean();
        assert_eq!(avg, 4.);
    }

    #[test]
    fn var_calculated_for_f64s() {
        let nums = vec![5., 5., 10., 3.];
        let var = nums.var();
        assert_eq!(var, 6.6875);
    }

    #[test]
    fn var_calculated_for_i32s() {
        let nums = vec![5, 5, 10, 3];
        let var = nums.var();
        assert_eq!(var, 6.6875);
    }

    #[test]
    fn std_calculated_for_i32s() {
        let nums = vec![5, 5, 10, 3];
        let var = nums.var();
        assert_eq!(nums.std(), var.sqrt());
    }

    #[test]
    fn std_calculated_for_f64s() {
        let nums = vec![5., 5., 10., 3.];
        let var = nums.var();
        assert_eq!(nums.std(), var.sqrt());
    }

    #[test]
    fn covar_calculated_for_f64s() {
        let x1 = vec![10., 3., 19., 8., 7.];
        let x2 = vec![13., 4., 21., 8., 3.];
        let covar = x1.covar(&x2).unwrap();
        assert_eq!(covar, 41.35);
    }

    #[test]
    fn diff_vec_lens_produces_error_for_covar() {
        let x1 = vec![10., 3., 19., 8., 7.];
        let x2 = vec![13., 4., 21., 8.];
        let covar = x1.covar(&x2);
        assert!(covar.is_err());
    }
}
