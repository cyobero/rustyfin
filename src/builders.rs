use crate::stocks::{History, Stock};

/// Build a struct `T` from an insta
pub trait Builder<T = Self> {
    fn build(self) -> T;
}

#[derive(Clone, Debug)]
pub struct StockBuilder<'s> {
    pub symbol: Option<&'s str>,
}

#[derive(Clone, Debug)]
pub struct HistoryBuilder<S = Stock> {
    pub stock: Option<S>,
    pub period1: Option<usize>,
    pub period2: Option<usize>,
    pub interval: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct YahooFinanceBuilder<'y, E = History> {
    pub base_url: Option<&'y str>,
    pub events: Option<E>,
}
