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
                    $($k: $v),*,
                    #[cfg(feature = "test")]
                    test_url: $crate::test_url::TestUrlHold::default(),
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
