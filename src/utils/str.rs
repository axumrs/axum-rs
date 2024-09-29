use bitflags::bitflags;
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref UPPERCASE_DICT: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    static ref LOWERCASE_DICT: &'static str = "abcdefghijklmnopqrstuvwxyz";
    static ref NUMBERS_DICT: &'static str = "0123456789";
}

bitflags! {
    pub struct RandomType:u8 {
        const Uppercase = 1;
        const Lowercase = 2;
        const Number = 4;
        const All = 1|2|4;
    }

}

pub fn rand_dict(ty: RandomType) -> String {
    let mut s = String::new();
    if ty.contains(RandomType::Uppercase) {
        s.push_str(*UPPERCASE_DICT);
    }
    if ty.contains(RandomType::Lowercase) {
        s.push_str(*LOWERCASE_DICT);
    }
    if ty.contains(RandomType::Number) {
        s.push_str(*NUMBERS_DICT);
    }
    s
}

pub fn rand_opt(len: usize, ty: Option<RandomType>) -> String {
    let ty = ty.unwrap_or(RandomType::All);
    let dict = rand_dict(ty);
    let mut s = String::with_capacity(len);
    for _ in 0..len {
        let idx = rand::thread_rng().gen_range(0..dict.len());
        s.push(dict.chars().nth(idx).unwrap_or_default());
    }
    s
}

pub fn rand(len: usize) -> String {
    rand_opt(len, None)
}

pub fn activation_code() -> String {
    rand_opt(20, Some(RandomType::All))
}

pub fn fixlen(s: &str, len: usize) -> &str {
    if utf8_slice::len(s) <= len {
        return s;
    }
    utf8_slice::slice(s, 0, len)
}

#[cfg(test)]
mod tests {
    use super::RandomType;

    #[test]
    fn test_random_type_has() {
        let ty = RandomType::All - RandomType::Uppercase | RandomType::Uppercase;
        assert!(ty.contains(RandomType::Uppercase));
    }

    #[test]
    fn test_rand_activation_code() {
        let s = super::activation_code();
        println!("{}", s);
    }
}
