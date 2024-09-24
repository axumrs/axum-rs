/// 生成新ID
pub fn new() -> String {
    xid::new().to_string()
}
