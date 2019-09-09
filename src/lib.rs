
pub mod order;
pub mod signature;

extern crate bigdecimal;
extern crate reqwest;
extern crate base64;
extern crate time;
extern crate hmac;
extern crate sha1;

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use crate::order::*;
use hyper::header::Headers;

header! { (FcAccessKey, "FC-ACCESS-KEY") => [String] }
header! { (FcAccessSignature, "FC-ACCESS-SIGNATURE") => [String] }
header! { (FcAccessTimestamp, "FC-ACCESS-TIMESTAMP") => [String] }

type Result<T> = std::result::Result<T, reqwest::Error>;

pub trait Function {
    fn symbols(&self) -> Result<ApiResponse<Vec<SymbolDescription>>>;

    fn coins(&self) -> Result<ApiResponse<Vec<String>>>;

    fn ordering(&self, order: &OrderRequest) -> Result<ApiResponse<String>>;

    fn cancel(&self, id: &str) -> Result<ApiResponse<bool>>;

    fn query(&self, condition: &OrderQuery) -> Result<ApiResponse<Vec<OrderInfo>>>;

    fn get(&self, id: &str) -> Result<ApiResponse<OrderInfo>>;

    fn get_match_result(&self, id: &str) -> Result<ApiResponse<Vec<MatchResult>>>;

    fn get_orderbook(&self, level: &str, coin: &str, currency: &str) -> Result<ApiResponse<Orderbook>>;
}

pub struct Fcoin {
    key: String,
    secret: String,
    uri: &'static str,
    http: reqwest::Client,
}

impl Fcoin {
    pub fn sandbox(key: &str, secret: &str) -> Fcoin {
        Fcoin::new(key, secret, "https://api-sandbox.fcoin.com/v2")
    }

    pub fn real(key: &str, secret: &str) -> Fcoin {
        Fcoin::new(key, secret, "https://api.fcoin.com/v2")
    }

    fn new(key: &str, secret: &str, host: &'static str) -> Fcoin {
        Fcoin {
            key: key.to_string(),
            secret: secret.to_string(),
            uri: host,
            http: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct SymbolDescription {
    pub name: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_decimal: u8,
    pub amount_decimal: u8,
}

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub status: i16,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl Function for Fcoin {

    fn symbols(&self) -> Result<ApiResponse<Vec<SymbolDescription>>> {
        let mut url = self.uri.to_string();
        let suffix = "/public/symbols";
        url += &suffix;
        let symbols: ApiResponse<Vec<SymbolDescription>> = self.http.get(&url).send()?.json()?;
        Ok(symbols)
    }

    fn coins(&self) -> Result<ApiResponse<Vec<String>>> {
        let mut url = self.uri.to_string();
        let suffix = "/public/currencies";
        url += &suffix;
        let coins: ApiResponse<Vec<String>> = self.http.get(&url).send()?.json()?;
        Ok(coins)
    }

    fn ordering(&self, order: &OrderRequest) -> Result<ApiResponse<String>> {
        let mut url = self.uri.to_string();
        let suffix = "/orders";
        url += &suffix;
        let current_time = time::get_time();
        let mut keys = vec!["amount".to_string(), "price".to_string(), "side".to_string(), "symbol".to_string(), "type".to_string()];
        let mut values = Vec::<String>::with_capacity(5);
        values.push(order.amount.to_string());
        values.push(order.price.to_string());
        values.push(order.buy_or_sell.to_string());
        values.push(order.symbol.to_string());
        values.push(order.instruction.to_string());
        let mut body = std::collections::HashMap::<&str, String>::new();
        body.insert("amount", order.amount.to_string());
        body.insert("price", order.price.to_string());
        body.insert("side", order.buy_or_sell.to_string());
        body.insert("symbol", order.symbol.to_string());
        body.insert("type", order.instruction.to_string());
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let sig = signature::sign("POST", "https://api.fcoin.com/v2/orders", timestamp, &mut keys, &mut values, &self.secret);
        let mut headers = Headers::new();
        headers.set(FcAccessKey(self.key.clone()));
        headers.set(FcAccessSignature(sig));
        headers.set(FcAccessTimestamp(timestamp.to_string()));
        let order_id: ApiResponse<String> = self.http.post(&url).headers(headers).json(&body).send()?.json()?;
        Ok(order_id)
    }

    fn cancel(&self, id: &str) -> Result<ApiResponse<bool>> {
        let mut url = self.uri.to_string();
        url += "/orders/";
        url += id;
        url += "/submit-cancel";
        let mut sig_prefix = "https://api.fcoin.com/v2".to_string();
        sig_prefix += "/orders/";
        sig_prefix += id;
        sig_prefix += "/submit-cancel";
        let current_time = time::get_time();
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let mut keys = Vec::<String>::new();
        let mut values = Vec::<String>::new();
        let sig = signature::sign("POST", &sig_prefix, timestamp, &mut keys, &mut values, &self.secret);
        let mut headers = Headers::new();
        headers.set(FcAccessKey(self.key.clone()));
        headers.set(FcAccessSignature(sig));
        headers.set(FcAccessTimestamp(timestamp.to_string()));
        let cancel: ApiResponse<bool> = self.http.post(&url).headers(headers).send()?.json()?;
        Ok(cancel)
    }

    fn query(&self, condition: &OrderQuery) -> Result<ApiResponse<Vec<OrderInfo>>> {
        let mut url = self.uri.to_string();
        url += "/orders";
        let mut sig_prefix = "https://api.fcoin.com/v2/orders".to_string();

        let mut query_str = "?".to_string();
        if let Some(after) = condition.after {
            query_str += "after=";
            query_str += &after.to_string();
            query_str += "&";
        }
        if let Some(before) = condition.before {
            query_str += "before=";
            query_str += &before.to_string();
            query_str += "&";
        }
        if let Some(limit) = condition.limit {
            query_str += "limit=";
            query_str += &limit.to_string();
            query_str += "&"
        }
        query_str += "states=";
        query_str += &condition.states;
        query_str += "&";
        query_str += "symbol=";
        query_str += &condition.symbol;
        url += &query_str;
        sig_prefix += &query_str;

        let mut keys = vec![];
        let mut values = vec![];
        let current_time = time::get_time();
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let sig = signature::sign("GET", &sig_prefix, timestamp, &mut keys, &mut values, &self.secret);
        let mut headers = Headers::new();
        headers.set(FcAccessKey(self.key.clone()));
        headers.set(FcAccessSignature(sig));
        headers.set(FcAccessTimestamp(timestamp.to_string()));
        let result: ApiResponse<Vec<OrderInfo>> = self.http.get(&url).headers(headers).send()?.json()?;
        Ok(result)
    }

    fn get(&self, id: &str) -> Result<ApiResponse<OrderInfo>> {
        let mut url = self.uri.to_string();
        url += "/orders/";
        url += id;
        let mut sig_prefix = "https://api.fcoin.com/v2/orders/".to_string();
        sig_prefix += id;
        let mut keys = vec![];
        let mut values = vec![];
        let current_time = time::get_time();
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let sig = signature::sign("GET", &sig_prefix, timestamp, &mut keys, &mut values, &self.secret);
        let mut headers = Headers::new();
        headers.set(FcAccessKey(self.key.clone()));
        headers.set(FcAccessSignature(sig));
        headers.set(FcAccessTimestamp(timestamp.to_string()));
        let result: ApiResponse<OrderInfo> = self.http.get(&url).headers(headers).send()?.json()?;
        Ok(result)
    }

    fn get_match_result(&self, id: &str) -> Result<ApiResponse<Vec<MatchResult>>> {
        let mut url = self.uri.to_string();
        url += "/orders/";
        url += id;
        url += "/match-results";
        let mut sig_prefix = "https://api.fcoin.com/v2/orders/".to_string();
        sig_prefix += id;
        sig_prefix += "/match-results";
        let mut keys = vec![];
        let mut values = vec![];
        let current_time = time::get_time();
        let timestamp: u64 = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
        let sig = signature::sign("GET", &sig_prefix, timestamp, &mut keys, &mut values, &self.secret);
        let mut headers = Headers::new();
        headers.set(FcAccessKey(self.key.clone()));
        headers.set(FcAccessSignature(sig));
        headers.set(FcAccessTimestamp(timestamp.to_string()));
        let mut response = self.http.get(&url).headers(headers).send()?;
        let result: ApiResponse<Vec<MatchResult>> = response.json()?;
        Ok(result)
    }

    fn get_orderbook(&self, level: &str, coin: &str, currency: &str) -> Result<ApiResponse<Orderbook>> {
        let mut url = self.uri.to_string();
        url +=  "/market/depth/";
        url += level;
        url = url + "/" + coin + currency;
        let mut response = self.http.get(&url).send()?;
        let result: ApiResponse<Orderbook> = response.json()?;
        Ok(result)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_currencies() {
        let fcoin = Fcoin::real("sk", "sd");
        let v = fcoin.coins().unwrap();
        assert_eq!(v.status, 0);
        assert_ne!(v.data, None);
    }

    #[test]
    fn test_symbols() {
        let fcoin = Fcoin::real("sk", "sd");
        let v = fcoin.symbols().unwrap();
        assert_eq!(v.status, 0);
        assert_ne!(v.data, None);
    }

    #[test]
    fn test_orderbook() {
        let fcoin = Fcoin::real("sk", "sd");
        let v = fcoin.get_orderbook("L20", "btc", "usdt").unwrap();
        assert_eq!(v.status, 0);
        assert_ne!(v.data, None);
    }
}
