use finance::builders::Builder;
use finance::stocks::*;

fn main() {
    let amd = Stock::builder().symbol("AMD").build();
    let history = History::builder()
        .stock(amd)
        .period1(1987)
        .period2(2021)
        .interval(1)
        .build();

    let yf = YahooFinance::builder().event(history).build();
    println!("endpoint {}", &yf.endpoint());
}
