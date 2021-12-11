use std::fmt;
use std::iter::IntoIterator;

pub trait MovingAverage<I = Self>
where
    I: IntoIterator,
{
    // Calculate the simple moving average over n `periods`
    fn sma(&self, periods: usize) -> I;
    // Calculate the exponential moving average over n `periods`.
    fn ema(&self, periods: usize) -> I;
}

impl MovingAverage for Vec<f64> {
    fn sma(&self, periods: usize) -> Vec<f64> {
        let mut sum = 0f64;
        let mut ma = Vec::<f64>::new();
        for i in 0..self.len() {
            if i >= periods {
                ma.push(sum / periods as f64);
                sum -= self[i - periods];
            }
            sum += self[i];
        }
        ma
    }

    fn ema(&self, periods: usize) -> Vec<f64> {
        let mut ma = Vec::<f64>::new();
        let mut sum = 0f64;
        let multiplier = 2. / (periods as f64 + 1.);
        for i in 0..self.len() {
            if i == periods {
                ma.push(sum / periods as f64);
                sum -= self[i];
            } else if i > periods {
                let val = (1. - multiplier) * ma.last().unwrap() + multiplier * self[i];
                ma.push(val);
            }
            sum += self[i];
        }
        ma
    }
}
