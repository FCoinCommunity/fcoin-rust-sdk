use super::base64;
use hmac::{Hmac, Mac};
use sha1::Sha1;

pub fn sign(method: &'static str,
           url: &str,
           timestamp: u64,
           keys: &mut Vec<String>,
           values: &mut Vec<String>,
           sk: &str) -> String {
    let mut sig = method.to_string();
    sig += &url;
    let t = timestamp.to_string();
    sig += &t;
    for x in 0..keys.len() {
        sig += &keys[x];
        sig += "=";
        sig += &values[x];
        sig += "&";
    }
    if sig.ends_with("&") {
        sig.pop();
    }
    let pre_hmac = base64::encode(&sig);
    dg(pre_hmac.as_bytes(), sk)
}

fn dg(bytes: &[u8], sk: &str) -> String {
    let mut mac = Hmac::<Sha1>::new(sk.as_bytes()).unwrap();
    mac.input(bytes);
    let hmac_bytes = mac.result().code();
    base64::encode(&hmac_bytes)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sig() {
        let t: u64 = 1523069544359;
        let mut keys: Vec<String> = vec!["amount".to_string(), "price".to_string(), "side".to_string(), "symbol".to_string(), "type".to_string()];
        let mut values: Vec<String> = vec!["100.0".to_string(), "100.0".to_string(), "buy".to_string(), "btcusdt".to_string(), "limit".to_string()];
        let r = sign("POST", "https://api.fcoin.com/v2/orders", t, &mut keys, &mut values, "3600d0a74aa3410fb3b1996cca2419c8");
        assert_eq!("DeP6oftldIrys06uq3B7Lkh3a0U=", r);
    }
}
