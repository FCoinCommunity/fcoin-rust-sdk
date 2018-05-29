use order::*;
use signature;
use reqwest;
use std;
use hyper::header::Headers;
use time;

header! { (FcAccessKey, "FC-ACCESS-KEY") => [String] }
header! { (FcAccessSignature, "FC-ACCESS-KEY") => [String] }
header! { (FcAccessTimestamp, "FC-ACCESS-TIMESTAMP") => [String] }

type Result<T> = std::result::Result<T, reqwest::Error>;

pub trait Function {
    fn symbols(&self) -> Result<Vec<SymbolDescription>>;

    fn coins(&self) -> Result<Vec<String>>;

    fn ordering(&self, order: &OrderRequest) -> Result<String>;

    fn cancel(&self, id: &str) -> Result<bool>;
}

pub struct Fcoin {
    key: String,
    secret: String,
    uri: &'static str,
    http: reqwest::Client,
}

impl Fcoin {
    pub fn sandbox(key: String, secret: String) -> Fcoin {
        Fcoin::new(key, secret, "https://api-sandbox.fcoin.com/v2/")
    }

    pub fn real(key: String, secret: String) -> Fcoin {
        Fcoin::new(key, secret, "https://api.fcoin.com/v2/")
    }

    fn new(key: String, secret: String, host: &'static str) -> Fcoin {
        Fcoin {
            key: key,
            secret: secret,
            uri: host,
            http: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse<T> {
    status: i16,
    data: T
}

#[derive(Deserialize)]
pub struct SymbolDescription {
    name: String,
    base_currency: String,
    quote_currency: String,
    price_decimal: u8,
    amount_decimal: u8,
}


impl Fcoin {

    fn symbols(&self) -> Result<Vec<SymbolDescription>> {
        let mut url = self.uri.to_string();
        let suffix = "public/symbols";
        url += &suffix;
        let symbols: ApiResponse<Vec<SymbolDescription>> = self.http.get(&url).send()?.json()?;
        Ok(symbols.data)
    }

    fn coins(&self) -> Result<Vec<String>> {
        let mut url = self.uri.to_string();
        let suffix = "public/currencies";
        url += &suffix;
        let coins: ApiResponse<Vec<String>> = self.http.get(&url).send()?.json()?;
        Ok(coins.data)
    }

    fn ordering(&self, order: &OrderRequest) -> Result<String> {
        let mut url = self.uri.to_string();
        let suffix = "orders";
        url += &suffix;
        let mut headers = Headers::new();
        let current_time = time::get_time();
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let mut keys = vec!["amount".to_string(), "price".to_string(), "side".to_string(), "symbol".to_string(), "type".to_string()];
        let mut values = Vec::<String>::with_capacity(5);
        values.push(order.amount.to_string());
        values.push(order.price.to_string());
        values.push(order.buy_or_sell.to_string());
        values.push(order.symbol.to_string());
        values.push(order.instruction.to_string());
        let sig = signature::sign("POST", &url, timestamp, &mut keys, &mut values, &self.secret);
        headers.set(FcAccessKey(self.key.clone()));
        headers.set(FcAccessSignature(sig));
        headers.set(FcAccessTimestamp(timestamp.to_string()));
        let orderId: ApiResponse<String> = self.http.post(&url).headers(headers).send()?.json()?;
        Ok(orderId.data)
    }

    fn cancel(&self, id: &str) -> Result<bool> {
        let mut url = self.uri.to_string();
        url += "orders/";
        url += id;
        url += "/submit-cancel";
        let mut headers = Headers::new();
        let current_time = time::get_time();
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let mut keys = Vec::<String>::new();
        let mut values = Vec::<String>::new();
        let sig = signature::sign("POST", &url, timestamp, &mut keys, &mut values, &self.secret);
        self.http.post(&url).send()?.json()?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_currencies() {
        let fcoin = Fcoin::real("sk".to_string(), "sd".to_string());
        let v = fcoin.coins().unwrap();
        assert_ne!(v.len(), 0);
    }

    #[test]
    fn test_symbols() {
        let fcoin = Fcoin::real("sk".to_string(), "sd".to_string());
        let v = fcoin.symbols().unwrap();
        assert_ne!(v.len(), 0);
    }
}
