use bcrypt::DEFAULT_COST;

use crate::{Error, Result};

pub fn hash(pwd: &str) -> Result<String> {
    bcrypt::hash(pwd, DEFAULT_COST).map_err(Error::from)
}
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool> {
    bcrypt::verify(pwd, hashed_pwd).map_err(Error::from)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_password_hash() {
        let pwd = "axum.rs";
        println!("{}", super::hash(pwd).unwrap());
    }
}
