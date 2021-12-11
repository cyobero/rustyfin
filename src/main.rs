use finance::builders::Builder;
use finance::ma::MovingAverage;
use finance::stocks::*;

fn main() {
    let nums = vec![5., 10., 3., 9., 8., 7., 2.5, 8.5, 1.9, 2.2];
    let n = 2usize;
    let sma = nums.sma(n);
    let ema = nums.ema(n);

    println!("nums: {:?}", nums);
    println!("simple {}-period moving average: {:?}", n, sma);
    println!("exponential {}-period moving average: {:?}", n, ema);
}
