use url::Url;

pub trait TestUrl {
    fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self;
    fn get_test_url(&self) -> Option<Url>;
}

#[derive(Default, Debug)]
pub struct TestUrlHold {
    test_url: Option<String>,
}

impl TestUrlHold {
    pub fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.test_url = Some(url.into());
        self
    }

    pub fn get_test_url(&self) -> Option<Url> {
        self.test_url.as_ref().map(|url| Url::parse(url).unwrap())
    }
}

#[cfg(feature = "test")]
macro_rules! impl_testurl {
    ($type:ty) => {
        #[cfg(feature = "test")]
        impl $crate::test_url::TestUrl for $type {
            fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
                self.test_url.with_url(url);
                self
            }

            fn get_test_url(&self) -> Option<Url> {
                self.test_url.get_test_url()
            }
        }
    };
}
