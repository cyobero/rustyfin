pub trait CentralMoment<T: PartialEq> {
    fn mean(&self) -> T;
    fn variance(&self) -> T;
}

impl CentralMoment<f64> for Vec<f64> {
    fn mean(&self) -> f64 {
        let mut sum = 0f64;
        for i in 0..self.len() {
            sum += self[i];
        }
        sum / self.len() as f64
    }

    fn variance(&self) -> f64 {
        let mut sum = 0f64;
        for i in 0..self.len() {
            sum += (self[i] - self.mean()).powf(2.);
        }
        sum / self.len() as f64
    }
}
