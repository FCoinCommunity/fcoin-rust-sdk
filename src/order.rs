use bigdecimal::BigDecimal;
use std::str::FromStr;

pub struct OrderRequest<'a> {
    pub symbol: &'a str,
    pub instruction: &'a str,
    pub buy_or_sell: &'static str,
    pub price: BigDecimal,
    pub amount: BigDecimal,
}

impl<'a> OrderRequest<'a> {
    pub fn sell_limit(symbol: &'a str, amount: BigDecimal, price: BigDecimal) -> OrderRequest<'a> {
        OrderRequest {
            symbol: symbol,
            instruction: "limit",
            price: price,
            amount: amount,
            buy_or_sell: "sell",
        }
    }

    pub fn sell_market(symbol: &'a str, amount: BigDecimal) -> OrderRequest<'a> {
        OrderRequest {
            symbol: symbol,
            instruction: "market",
            price: BigDecimal::from_str("0.0").unwrap(),
            amount: amount,
            buy_or_sell: "sell",
        }
    }

    pub fn buy_limit(symbol: &'a str, amount: BigDecimal, price: BigDecimal) -> OrderRequest<'a> {
        OrderRequest {
            symbol: symbol,
            instruction: "limit",
            price: price,
            amount: amount,
            buy_or_sell: "buy",
        }
    }

    pub fn buy_market(symbol: &'a str, amount: BigDecimal) -> OrderRequest<'a> {
        OrderRequest {
            symbol: symbol,
            instruction: "market",
            price: BigDecimal::from_str("0.0").unwrap(),
            amount: amount,
            buy_or_sell: "buy",
        }
    }
}

#[cfg(test)]
mod tests {

    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn test_order() {
        // let buy_limit = super::OrderRequest::buy_limit(
        //     "usdtbtc",
        //     BigDecimal::from_str("100.0").unwrap(),
        //     BigDecimal::from_str("1.00000").unwrap(),
        // );
        // let sell_limit = super::OrderRequest::sell_limit(
        //     "usdtbtc",
        //     BigDecimal::from_str("101.00").unwrap(),
        //     BigDecimal::from_str("0.99999").unwrap(),
        // );
        // let buy_market =
        //     super::OrderRequest::buy_market("usdtbtc", BigDecimal::from_str("1.2000").unwrap());
        // let sell_market =
        //     super::OrderRequest::sell_market("usdtbtc", BigDecimal::from_str("1.09999").unwrap());
    }
}
