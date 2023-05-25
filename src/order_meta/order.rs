use sha2::{Digest, Sha256};

use crate::{Error, Result};

/// 生成20个字符的订单号
pub fn number() -> String {
    xid::new().to_string().to_uppercase()
}

/// 根据订单号生成订单识别码 -> (64个字符完整识别码, 7个字符的简短识别码)
pub fn code(num: &str) -> Result<(String, String)> {
    let mut hasher = Sha256::new();
    hasher.update(num);
    let hash = hasher.finalize();
    let mut buf = [0u8; 64];
    let hash = base16ct::lower::encode_str(hash.as_slice(), &mut buf).map_err(Error::from)?;
    let short_hash = &hash[..7];
    Ok((hash.to_string(), short_hash.to_string()))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_gen_order_no() {
        for _ in 0..10 {
            let on = super::number();
            println!("{} {}", on, on.len());
        }
    }
    #[test]
    fn test_gen_order_code() {
        for _ in 0..10 {
            let on = super::number();
            let oc = super::code(&on);
            println!("{} {:?}", on, oc);
            let oc = oc.unwrap();
            println!("{}, {}", oc.0.len(), oc.1.len());
        }
    }
}
