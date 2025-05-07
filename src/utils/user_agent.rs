use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref USER_AGENT_LIST: Vec<&'static str> = vec![
        // Firefox @ win11
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0",
        // Opera @ win11
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 OPR/114.0.0.0",
        // Chrome @ win11
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
        // Vivaldi @ win11
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Vivaldi/6.9.3447.37"
    ];
}

pub fn get() -> &'static str {
    let idx = rand::rng().random_range(0..USER_AGENT_LIST.len());
    USER_AGENT_LIST[idx]
}
