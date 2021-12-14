use finance::ma::MovingAverage;

fn main() {
    let nums_f = vec![5., 7., 8., 6., 5., 5.5, 4.5];
    let nums_i = vec![10, 3, 19, 8, 7, 14, 21, 20, 29, 19, 8, 10, 10, 11, 12];
    println!("nums_f: {:?}", nums_f);
    println!("2-day SMA for nums_f: {:?}", nums_f.sma(2));
    println!(
        "2-day EMA for nums_f: {:?}",
        vec![2., 4., 6., 8., 12.].ema(2)
    );

    println!("nums_i: {:?}", nums_i);
    println!("5-day SMA for nums_i: {:?}", nums_i.sma(5));
    println!("5-day EMA for nums_i: {:?}", nums_i.ema(5));
}
