use askama::Template;

#[derive(Template)]
#[template(path = "exchange-rate.html")]
pub struct ExchangeRate {
    pub sol: f64,
    pub usd: f64,
    pub rate: f64,
}
