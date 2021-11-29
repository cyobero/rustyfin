use finance::stocks::{HistoryBuilder, Stock};

fn main() {
    let amd = Stock::builder().symbol("AMD").build();
    let history = HistoryBuilder::new()
        .stock(amd)
        .period1(1987)
        .period2(2021)
        .interval(1)
        .build();

    println!("{:?}", history);
}
