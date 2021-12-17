use finance::{CentralMoment, MovingAverage, Volatility};

fn main() {
    //    let nums_f = vec![5., 7., 8., 6., 5., 5.5, 4.5];
    //let nums_i = vec![10, 3, 19, 8, 7, 14, 21, 20, 29, 19, 8, 10, 10, 11, 12];
    //println!("nums_f: {:?}", nums_f);
    //println!("2-day SMA for nums_f: {:?}", nums_f.sma(2));
    //println!(
    //"2-day EMA for nums_f: {:?}",
    //vec![2., 4., 6., 8., 12.].ema(2)
    //);

    //println!("nums_i: {:?}", nums_i);
    //println!("5-day SMA for nums_i: {:?}", nums_i.sma(5));
    //println!("5-day EMA for nums_i: {:?}\n", nums_i.ema(5));

    //println!("range of nums_i: {:?}", nums_i.range_volatility());
    //    println!("range of nums_f: {:?}", nums_f.range_volatility());
    let nums = vec![5., 5., 10., 3.];
    let n = nums.len();
    let mean = nums.mean();
    let mut ss = 0f64;
    println!("nums: {:?}", nums);
    println!("mean: {}\n", mean);
    for x in nums {
        let sq_diff = (x - mean).powf(2.);
        println!("(x - mean)^2 = {}\t", (x - mean).powf(2.));
        ss += sq_diff;
        println!("ss: {}\n", ss);
    }
    let var = ss / (n as f64);
    println!("variance: {}", var);
}
