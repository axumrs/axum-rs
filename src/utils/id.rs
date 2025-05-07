/// 生成新ID
pub fn new() -> String {
    xid::new().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_id() {
        let id = super::new();
        println!("id: {id}");
    }
}
