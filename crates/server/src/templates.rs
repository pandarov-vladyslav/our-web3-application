use askama::Template;

#[derive(Template)]
#[template(path = "exchange-rate.html")]
pub struct ExchangeRate<'a> {
    pub sol: &'a str,
    pub usd: &'a str,
    pub rate: &'a str,
}
