use url::Url;

pub trait TestUrl {
    fn with_url(self, port: Option<u16>, endpoint: Option<String>) -> Self;
}

#[derive(Default, Debug)]
pub struct TestUrlHold {
    endpoint: Option<String>,
    port: Option<u16>,
}

impl TestUrlHold {
    /// defualt 'mock'
    pub fn with_endpoint(&mut self, endpoint: Option<String>) -> &mut Self {
        if let Some(endpoint) = endpoint {
            self.endpoint = Some(endpoint);
        } else {
            self.endpoint = Some("mock".to_string());
        }
        self
    }

    /// default '8080'
    pub fn with_port(&mut self, port: Option<u16>) -> &mut Self {
        if let Some(port) = port {
            self.port = Some(port);
        } else {
            self.port = Some(8080);
        }
        self
    }

    pub fn from_url(&self, url: &Url) -> Result<Url, crate::Error> {
        if self.endpoint.is_none() && self.port.is_none() {
            Err(crate::Error::MissingTestUrl)
        } else {
            let mut test_url = Url::parse("http://localhost:8080").unwrap();

            test_url.set_port(self.port).unwrap();
            let paths = url
                .path_segments()
                .unwrap()
                .skip(1)
                .fold(self.endpoint.clone().unwrap(), |acc, x| {
                    format!("{acc}/{x}")
                });
            test_url.set_path(&paths);
            test_url.set_query(url.query());

            Ok(test_url)
        }
    }
}
