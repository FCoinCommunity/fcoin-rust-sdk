use base64;
use hmacsha1;

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
    for x in 0..keys.len() - 1 {
        let k = keys.remove(0);
        let v = values.remove(0);
        sig += &k;
        sig += "=";
        sig += &v;
        sig += "&";
    }
    let k = keys.remove(0);
    let v = values.remove(0);
    sig += &k;
    sig += "=";
    sig += &v;
    let pre_hmac = base64::encode(&sig);
    dg(pre_hmac.as_bytes(), sk)
}

fn dg(bytes: &[u8], sk: &str) -> String {
    let hmac_bytes = hmacsha1::hmac_sha1(sk.as_bytes(), bytes);
    let mut buffer = "".to_string();
    for b in &hmac_bytes {
        buffer += &format!("{:02x}", b);
    }
    base64::encode(&buffer)
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
        assert_eq!("MGRlM2ZhYTFmYjY1NzQ4YWYyYjM0ZWFlYWI3MDdiMmU0ODc3NmI0NQ==", &r);
    }
}
