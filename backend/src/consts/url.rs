use once_cell::sync::Lazy;

pub const MOCK_HOST: &str = "127.0.0.1";
pub const MOCK_PORT: u16 = 3000;
pub static MOCK_BASE_URL: Lazy<String> =
    Lazy::new(|| format!("http://{}:{}", MOCK_HOST, MOCK_PORT));
