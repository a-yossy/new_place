#[cfg(test)]
pub fn get_base_url(_base_url: &str) -> &'static str {
    use crate::consts::url::MOCK_BASE_URL;

    MOCK_BASE_URL.as_str()
}

#[cfg(not(test))]
pub fn get_base_url(base_url: &str) -> &str {
    base_url
}
