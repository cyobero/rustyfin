use reqwest::{self, Client};

pub mod calc;

pub mod stocks {
    use serde::Deserialize;

    /// Take a struct `T` and return the final endpoint of the URL request
    pub trait Endpoint<'e, T: Deserialize<'e>> {
        fn endpoint(&self) -> T;
    }

    pub trait Builder<T = Self> {
        fn build(self) -> T;
    }

    // Builder structs
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

    #[derive(Clone, Debug)]
    pub struct Stock<T = String> {
        symbol: T,
    }

    #[derive(Clone, Debug)]
    pub struct History<P = usize, U = usize, S = Stock>
    where
        U: Sized + PartialEq,
    {
        stock: S,
        period1: P,
        period2: P,
        interval: U,
    }

    #[derive(Clone, Debug)]
    pub struct YahooFinance<H = History> {
        base_url: String,
        events: H,
    }

    mod impls {
        use super::*;

        impl<'a> Endpoint<'a, String> for YahooFinance {
            fn endpoint(&self) -> String {
                format!("{}/{}/history?", &self.base_url, &self.events.stock.symbol)
            }
        }

        impl<'y> YahooFinance {
            pub fn new() -> Self {
                YahooFinance {
                    base_url: String::from("https://finance.yahoo.com/quote/"),
                    events: History::new(),
                }
            }

            pub fn builder() -> YahooFinanceBuilder<'y> {
                YahooFinanceBuilder::new()
            }
        }

        impl<'yb> YahooFinanceBuilder<'yb> {
            pub fn new() -> Self {
                YahooFinanceBuilder {
                    base_url: Some("https://finance.yahoo.com/quote/"),
                    events: Some(History::new()),
                }
            }

            pub fn base_url(mut self, url: &'yb str) -> Self {
                self.base_url = Some(url);
                self
            }

            pub fn event(mut self, event: History) -> Self {
                self.events = Some(event);
                self
            }
        }

        impl Builder<YahooFinance> for YahooFinanceBuilder<'_> {
            fn build(self) -> YahooFinance {
                YahooFinance {
                    base_url: self.base_url.unwrap().to_owned(),
                    events: self.events.unwrap(),
                }
            }
        }

        impl<'s> Stock {
            pub fn new() -> Self {
                Stock {
                    symbol: String::new(),
                }
            }

            pub fn builder() -> StockBuilder<'s> {
                StockBuilder::new()
            }
        }

        impl<'sb> StockBuilder<'sb> {
            pub fn new() -> Self {
                StockBuilder { symbol: None }
            }

            pub fn symbol(mut self, symbl: &'sb str) -> Self {
                self.symbol = Some(symbl);
                self
            }
        }

        impl Builder<Stock> for StockBuilder<'_> {
            fn build(self) -> Stock {
                Stock {
                    symbol: self.symbol.unwrap().to_owned(),
                }
            }
        }

        impl History {
            pub fn new() -> History {
                History {
                    stock: Stock::new(),
                    period1: 0,
                    period2: 0,
                    interval: 0,
                }
            }

            pub fn builder() -> HistoryBuilder {
                HistoryBuilder::new()
            }
        }

        impl<'hb> HistoryBuilder {
            pub fn new() -> Self {
                HistoryBuilder {
                    stock: None,
                    period1: None,
                    period2: None,
                    interval: None,
                }
            }

            pub fn stock(mut self, stock: Stock) -> Self {
                self.stock = Some(stock);
                self
            }

            pub fn period1(mut self, period: usize) -> Self {
                self.period1 = Some(period);
                self
            }

            pub fn period2(mut self, period: usize) -> Self {
                self.period2 = Some(period);
                self
            }

            pub fn interval(mut self, interval: usize) -> Self {
                self.interval = Some(interval);
                self
            }
        }

        impl Builder<History> for HistoryBuilder {
            fn build(self) -> History {
                History {
                    stock: self.stock.unwrap(),
                    period1: self.period1.unwrap(),
                    period2: self.period2.unwrap(),
                    interval: self.interval.unwrap(),
                }
            }
        }
    }
}
