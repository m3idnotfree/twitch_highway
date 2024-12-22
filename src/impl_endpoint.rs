#[macro_export]
macro_rules! impl_default_header {
    () => {
        fn headers(&self) -> asknothingx2_util::api::HeaderMap {
            asknothingx2_util::api::HeaderBuilder::new()
                .authorization("Bearer", self.access_token.secret().as_str())
                .client_id(self.client_id.as_str())
                .build()
        }
    };
}

#[macro_export]
macro_rules! impl_api_request_method {
    ($method:ident) => {
        fn method(&self) -> asknothingx2_util::api::Method {
            asknothingx2_util::api::Method::$method
        }
    };
}

#[macro_export]
macro_rules! impl_endpoint {
    (
        $(#[$meta:meta])*
        $name:ident {
           $(
              $(#[$field_meta:meta])*
               $ident:ident: $field:ty
           ), *$(,)?
        };
        new = {
            params = {
                $($param:ident: $type:ty),* $(,)?
            },
            init = {
                $($k:ident: $v:expr),* $(,)?
            }
        },
        url = [$($segment:literal),* $(,)?] $(,)?
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        pub struct $name {
            access_token: asknothingx2_util::oauth::AccessToken,
            client_id: asknothingx2_util::oauth::ClientId,
            // $($ident: $field),*,
            $(
                $(#[$field_meta])*
                $ident: $field,
            )*

            #[cfg(feature = "test")]
            test_url: $crate::test_url::TestUrlHold,
        }

        impl $name {
            pub fn new(
            access_token: asknothingx2_util::oauth::AccessToken,
            client_id: asknothingx2_util::oauth::ClientId,
            $($param: $type),*
            ) -> Self {
                Self {
                    access_token,
                    client_id,
                    #[cfg(feature = "test")]
                    test_url: $crate::test_url::TestUrlHold::default(),
                    $($k: $v),*
                }

           }

            fn get_url(&self) -> Url {
                #[cfg(feature = "test")]
                if let Some(url) = self.test_url.get_test_url() {
                    return url;
                }

                let mut url = Url::parse($crate::TWITCH_API_BASE).unwrap();
                url.path_segments_mut().unwrap().extend([$($segment),*]);
                url
            }
        }

         #[cfg(feature = "test")]
         $crate::impl_testurl!($name);
    };
}
