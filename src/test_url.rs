use url::Url;

pub trait TestUrl {
    fn with_url<T: Into<String>>(self, url: T) -> Self;
    fn get_test_url(&self) -> Result<Url, crate::Error>;
}

#[derive(Default, Debug)]
pub struct TestUrlHold(Option<String>);

impl TestUrlHold {
    pub fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.0 = Some(url.into());
        self
    }

    pub fn get_test_url(&self) -> Result<Url, crate::Error> {
        self.0
            .as_ref()
            .ok_or(crate::Error::MissingTestUrl)
            .and_then(|url| Ok(Url::parse(url)?))
    }

    pub fn users_url(port: Option<u16>) -> String {
        if let Some(port) = port {
            return format!("http://localhost:{}/mock/users", port);
        }
        "http://localhost:8080/mock/users".to_string()
    }
    pub fn eventsub_url(port: Option<u16>) -> String {
        if let Some(port) = port {
            return format!("http://localhost:{}/eventsub/subscriptions", port);
        }
        "http://localhost:8080/eventsub/subscriptions".to_string()
    }
}
