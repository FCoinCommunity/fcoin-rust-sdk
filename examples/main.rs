extern crate fcoin;
extern crate bigdecimal;

use fcoin::*;
use std::str::FromStr;
use bigdecimal::BigDecimal;

pub fn main() {
    let api = Fcoin::sandbox("c62eef2b6d5748d6b592b3ec5eda00f9", "890bf56c2c3646b1a752e94a3be2d042");
    let rsp: ApiResponse<Vec<String>> = api.coins().unwrap();
    if rsp.status == 0 {
        if let Some(v) = rsp.data {
            println!("{:?}", v);
        }
    } else {
        println!("error: status={} ", rsp.status);
        if let Some(m) = rsp.msg {
            println!("{}", m);
        }
    }
    let condition: order::OrderQuery = order::OrderQuery {
        symbol: None,
        states: None,
        before: None,
        after: None,
        limit: None,
    };
    let rsp: ApiResponse<Vec<order::OrderInfo>> = api.query(&condition).unwrap();
    if rsp.status == 0 {
        if let Some(v) = rsp.data {
            println!("{}", v.len());
        }
    } else {
        println!("error: status={} ", rsp.status);
        if let Some(m) = rsp.msg {
            println!("{}", m);
        }
    }
    let order = order::OrderRequest::sell_limit("btcusdt", BigDecimal::from_str("1.0").unwrap(), BigDecimal::from_str("1000.99").unwrap());
    let rsp = api.ordering(&order).unwrap();
    if rsp.status == 0 {
        if let Some(v) = rsp.data {
            println!("order submitted, id: {:?}", v);
            api.cancel(&v).unwrap();
        }
    } else {
        println!("error: status={} ", rsp.status);
        if let Some(m) = rsp.msg {
            println!("{}", m);
        }
    }
}
