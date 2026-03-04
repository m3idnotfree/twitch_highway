pub trait HeaderAccess {
    fn get_header(&self, name: &str) -> Option<&str>;
}

#[cfg(feature = "webhook-axum")]
impl HeaderAccess for http::HeaderMap {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.get(name)?.to_str().ok()
    }
}

#[cfg(feature = "webhook-actix")]
impl HeaderAccess for actix_http::header::HeaderMap {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.get(name)?.to_str().ok()
    }
}

impl HeaderAccess for &[(&str, &str)] {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }
}

impl HeaderAccess for Vec<(String, String)> {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}
