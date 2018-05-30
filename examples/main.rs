extern crate fcoin;
extern crate bigdecimal;

use fcoin::*;
use std::str::FromStr;
use bigdecimal::BigDecimal;

pub fn main() {
    let api = Fcoin::sandbox("your_api_key", "your_api_secret");
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
        symbol: "btcusdt".to_string(),
        states: "filled".to_string(),
        before: None,
        after: None,
        limit: None,
    };
    let rsp: ApiResponse<Vec<order::OrderInfo>> = api.query(&condition).unwrap();
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
    let order = order::OrderRequest::buy_limit("btcusdt", BigDecimal::from_str("1.80").unwrap(), BigDecimal::from_str("7448.92").unwrap());
    let rsp = api.ordering(&order).unwrap();
    if rsp.status == 0 {
        if let Some(v) = rsp.data {
            println!("order submitted, id: {:?}", v);
            api.cancel(&v).unwrap();
            let info = api.get(&v).unwrap();
            if let Some(i) = info.data {
                println!("{:?}", i);
            } else {
                println!("error to get order, status={}", info.status);
                if let Some(m) = info.msg {
                    println!("{}", m);
                }
            }
            let match_result = api.get_match_result(&v).unwrap();
            if let Some(mr) = match_result.data {
                println!("{:?}", mr);
            } else {
                println!("error to get match result, status={}", match_result.status);
                if let Some(m) = match_result.msg {
                    println!("{}", m);
                }
            }
        }
    } else {
        println!("error: status={} ", rsp.status);
        if let Some(m) = rsp.msg {
            println!("{}", m);
        }
    }
}
