/// # Field
/// - `key` -
/// - `convert` -
/// - `method` -
/// - `into` -
/// - `skip` -
///
/// # Generated Methods
///  - `new(required_params) -> Self`
///  - `optional_field(value) -> Self`
///  - `build() -> TwitchAPIRequest<ResponseType>`
///  - `send()` -> Result<Response>`
///  - `json() -> Result<ResponseType>`
#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "conduits",
    feature = "entitlements",
    feature = "eventsub",
    feature = "extensions",
    feature = "games",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "users",
))]
macro_rules! define_request_builder {
    (
        $(#[$meta:meta])*
        $name:ident<$lt:lifetime>{$(
            $(#[$opt_meta:meta])*
            $opt_f:ident: $opt_t:ty
            $([$($opt_config:tt)*])?
        ),* $(,)?
        } -> $return:ty;
        endpoint_type: $endpoint:ident,
        method: $method:ident,
        path: [$($path:expr),* $(,)?]
        $(, header: [$($header_config:tt)*] )?
        $(, body: $($body:expr)?)?
        $(,)?
    ) => {
        define_request_builder!(@impl
            $(#[$meta])*,
            $name,
            {
                <$lt>
                [],
            },
            {},
            {$(
                $(#[$opt_meta])*
                $opt_f: $opt_t
                $([$($opt_config)*])?
            ),*},
            endpoint_type: $endpoint,
            method: $method,
            path: [$($path),*],
            header: [$($($header_config:tt)*)?],
            body: $($body:expr)?,
            return: $return
        );
    };
    (
        $(#[$meta:meta])*
        $name:ident<$lt:lifetime, $($gen:ident $(: $bound:path)?),+>{$(
            $(#[$opt_meta:meta])*
            $opt_f:ident: $opt_t:ty
            $([$($opt_config:tt)*])?
        ),* $(,)?
        } -> $return:ty;
        endpoint_type: $endpoint:ident,
        method: $method:ident,
        path: [$($path:expr),* $(,)?]
        $(, header: [$($header_config:tt)*] )?
        $(, body: $($body:expr)?)?
        $(,)?
    ) => {
        define_request_builder!(@impl
            $(#[$meta])*,
            $name,
            {
                <$lt>
                [$($gen $(: $bound)?),+],
            },
            {},
            {$(
                $(#[$opt_meta])*
                $opt_f: $opt_t
                $([$($opt_config)*])?
            ),*},
            endpoint_type: $endpoint,
            method: $method,
            path: [$($path),*]
            header: [$($($header_config:tt)*)?],
            body: $($body:expr)?,
            return: $return
        );
    };
    (
        $(#[$meta:meta])*
        $name:ident<$lt:lifetime> {
            $(req: {$(
                $(#[$req_meta:meta])*
                $req_f:ident: $req_t:ty
                $([$($req_config:tt)*])?
            ),* $(,)?})? $(,)?

            $(opts: {$(
                $(#[$opt_meta:meta])*
                $opt_f:ident: $opt_t:ty
                $([$($opt_config:tt)*])?
            ),* $(,)?})? $(,)?
        } -> $return:ty;
        endpoint_type: $endpoint:ident,
        method: $method:ident,
        path: [$($path:expr),* $(,)?]
        $(, header: [$($header_config:tt)*] )?
        $(, body: $($body:expr)?)?
        $(,)?
    ) => {
        define_request_builder!(@impl
            $(#[$meta])*,
            $name,
            {
                <$lt>
                [],
            },
            {$($(
                $(#[$req_meta])*
                $req_f: $req_t
                $([$($req_config)*])?
            ),*)?},
            {$($(
                $(#[$opt_meta])*
                $opt_f: $opt_t
                $([$($opt_config)*])?
            ),*)?},
            endpoint_type: $endpoint,
            method: $method,
            path: [$($path),*],
            header: [$($($header_config)*)?],
            body: $($body:expr)?,
            return: $return
        );
    };
    //  (
    //     $(#[$meta:meta])*
    //     $name:ident<$lt:lifetime, $($gen:ident $(: $bound:path)?),+> {
    //         $(req: {$(
    //             $(#[$req_meta:meta])*
    //             $req_f:ident: $req_t:ty
    //             $([$($req_config:tt)*])?
    //         ),* $(,)?})? $(,)?
    //
    //         $(opts: {$(
    //             $(#[$opt_meta:meta])*
    //             $opt_f:ident: $opt_t:ty
    //             $([$($opt_config:tt)*])?
    //         ),* $(,)?})? $(,)?
    //     } -> $return:ty;
    //     endpoint_type: $endpoint:ident,
    //     method: $method:ident,
    //     path: [$($path:expr),* $(,)?]
    //     $(, header: [$($header_config:tt)*] )?
    //     $(, body: $($body:expr)?)?
    //     $(,)?
    // ) => {
    //     define_request_builder!(@impl
    //         $(#[$meta])*,
    //         $name,
    //         {
    //             <$lt>
    //             [$($gen $(: $bound)*),+],
    //         },
    //         {$($(
    //             $(#[$req_meta])*
    //             $req_f: $req_t
    //             $([$($req_config)*])?
    //         ),*)?},
    //         {$($(
    //             $(#[$opt_meta])*
    //             $opt_f: $opt_t
    //             $([$($opt_config)*])?
    //         ),*)?},
    //         endpoint_type: $endpoint,
    //         method: $method,
    //         path: [$($path),*]
    //         header: [$($($header_config:tt)*)?],
    //         body: $($body:expr)?,
    //         return: $return
    //     );
    // };


    (@impl $(#[$meta:meta])*,
        $name:ident,
        {$(
            <$($lt:lifetime)?>
            [$($($gen:ident $(: $bound:path)*),+)?],
        )?},
        {$($(
            $(#[$req_m:meta])*
            $req_f:ident: $req_t:ty
            $([$($req_config:tt)*])?
        ),+)?},
        {$($(
            $(#[$opt_m:meta])*
            $opt_f:ident: $opt_t:ty
            $([$($opt_config:tt)*])?
        ),+)?},
        endpoint_type: $endpoint:ident,
        method: $method:ident,
        path: [$($($path:expr),+ $(,)?)?],
        header: [$($($header_config:tt)+)?],
        body: $($body:expr)?,
        return: $return:ty
    ) => {
        $(#[$meta])*
        pub struct $name
       $( <$($lt)?, $($($gen $(: $bound)*),+)?>)?
        {
            api: $($(&$lt)?)? $crate::TwitchAPI,
            $($(
                $(#[$req_m])*
                $req_f: $req_t,
            )*)?
            $($(
                $opt_f: Option<$opt_t>,
            )*)?
        }

        impl$(<$($lt)?, $($($gen $(: $bound)*),+)?>)? $name$(<$($lt)?, $($($gen),+)?>)? {
            pub fn new(
                api: $($(&$lt)?)? $crate::TwitchAPI,

            $($(
                $req_f: define_request_builder!(@param_type $req_t $([$($req_config)*])?)),
            +)?) -> Self {
                Self {
                    api,
                    $($(
                        $req_f: define_request_builder!(@param_value $req_f $([$($req_config)*])?),
                    )+)?
                    $($(
                        $opt_f: None,
                    )+)?
                }
            }
            $($(
                define_request_builder!(@opt_method $(#[$opt_m])* $opt_f: $opt_t $(, [$($opt_config)*])?);
            )+)?

            pub fn build(self) -> $crate::request::TwitchAPIRequest<$return> {
                let mut url = self.api.build_url();
                let headers = define_request_builder!(@headers self, $($($header_config)*)?);
                let body = define_request_builder!(@body $($body)?);

                $(url.path_segments_mut().unwrap().extend([$($path),+]);)?

                let mut query = url.query_pairs_mut();

                $($(
                    define_request_builder!(@req_query query, $req_f, self.$req_f $(, [$($req_config)*])?);
                )+)?

                $($(
                    define_request_builder!(@opt_query query, $opt_f, self.$opt_f $(, [$($opt_config)*])?);
                )+)?

                drop(query);

                $crate::request::TwitchAPIRequest::new(
                    $crate::request::EndpointType::$endpoint,
                    url,
                    reqwest::Method::$method,
                    headers,
                    body,
                    self.api.client.clone(),
                )
            }

            pub async fn send(self) -> Result<reqwest::Response, $crate::Error> {
                self.build().send().await
            }

            pub async fn json(self) -> Result<$return, $crate::Error> {
                self.build().json().await
            }
        }
    };

    (@param_type $type:ty [$($config:tt)*]) => {
        define_request_builder!(@param_type_parse $type, into: false, $($config)*)
    };
    (@param_type $type:ty) => {
        $type
    };
    (@param_type_parse $type:ty, into: $into_flag:tt, into $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_type_parse $type, into: true $(, $($rest)*)?)
    };
    (@param_type_parse $type:ty, into: $into_flag:tt, skip $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_type_parse $type, into: $into_flag $(, $($rest)*)?)
    };

    (@param_type_parse $type:ty, into: $into_flag:tt, key = $key:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_type_parse $type, into: $into_flag $(, $($rest)*)?)
    };
    (@param_type_parse $type:ty, into: $into_flag:tt, convert = $conv:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_type_parse $type, into: $into_flag $(, $($rest)*)?)
    };
    (@param_type_parse $type:ty, into: $into_flag:tt, method = $method:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_type_parse $type, into: $into_flag $(, $($rest)*)?)
    };

    (@param_type_parse $type:ty, into: true $(,)?) => {
        impl Into<$type>
    };
    (@param_type_parse $type:ty, into: false $(,)?) => {
        $type
    };

    (@param_value $field:ident [$($config:tt)*]) => {
        define_request_builder!(@param_value_parse $field, into: false, $($config)*)
    };
    (@param_value $field:ident) => {
        $field
    };

    (@param_value_parse $field:ident, into: $into_flag:tt, into $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_value_parse $field, into: true $(, $($rest)*)?)
    };
    (@param_value_parse $field:ident, into: $into_flag:tt, skip $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_value_parse $field, into: $into_flag $(, $($rest)*)?)
    };

    (@param_value_parse $field:ident, into: $into_flag:tt, key = $key:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_value_parse $field, into: $into_flag $(, $($rest)*)?)
    };
    (@param_value_parse $field:ident, into: $into_flag:tt, convert = $conv:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_value_parse $field, into: $into_flag $(, $($rest)*)?)
    };
    (@param_value_parse $field:ident, into: $into_flag:tt, method = $method:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@param_value_parse $field, into: $into_flag $(, $($rest)*)?)
    };

    (@param_value_parse $field:ident, into: true $(,)?) => {
        $field.into()
    };

    (@param_value_parse $field:ident, into: false $(,)?) => {
        $field
    };

    (@opt_method $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, [$($config:tt)*]) => {
        define_request_builder!(@opt_method_parse $(#[$attr])* $opt_f: $opt_t, method: $opt_f, into: false, $($config)*);
    };

    (@opt_method $(#[$attr:meta])* $opt_f:ident: $opt_t:ty) => {
        $(#[$attr])*
        pub fn $opt_f(mut self, value: $opt_t) -> Self {
            self.$opt_f = Some(value);
            self
        }
    };

    (@opt_method_parse $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, method: $current_method:ident, into: $into_flag:tt, method = $new_method:ident $(, $($rest:tt)*)?) => {
        define_request_builder!(@opt_method_parse $(#[$attr])* $opt_f: $opt_t, method: $new_method, into: $into_flag $(, $($rest)*)?)
    };

    (@opt_method_parse $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, method: $current_method:ident, into: $into_flag:tt, into $(, $($rest:tt)*)?) => {
        define_request_builder!(@opt_method_parse $(#[$attr])* $opt_f: $opt_t, method: $current_method, into: true $(, $($rest)*)?)
    };

    (@opt_method_parse $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, method: $current_method:ident, into: $into_flag:tt, key = $key:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@opt_method_parse $(#[$attr])* $opt_f: $opt_t, method: $current_method, into: $into_flag $(, $($rest)*)?);
    };
    (@opt_method_parse $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, method: $current_method:ident, into: $into_flag:tt, convert = $conv:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@opt_method_parse $(#[$attr])* $opt_f: $opt_t, method: $current_method, into: $into_flag $(, $($rest)*)?);
    };

    (@opt_method_parse $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, method: $method_name:ident, into: true $(,)?) => {
        $(#[$attr])*
        pub fn $method_name(mut self, value: impl Into<$opt_t>) -> Self {
            self.$opt_f = Some(value.into());
            self
        }
    };

    (@opt_method_parse $(#[$attr:meta])* $opt_f:ident: $opt_t:ty, method: $method_name:ident, into: false $(,)?) => {
        $(#[$attr])*
        pub fn $method_name(mut self, value: $opt_t) -> Self {
            self.$opt_f = Some(value);
            self
        }
    };


    (@req_query $query:expr, $field:ident, $value:expr, [$($config:tt)*]) => {
        define_request_builder!(@req_query_parse $query, $field, $value, key: stringify!($field), convert: none, $($config)*)
    };
    (@req_query $query:expr, $field:ident, $value:expr) => {
        $query.append_pair(stringify!($field), $value);
    };

    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $current_key:expr, convert: $conv:tt, skip $(, $($rest:tt)*)?) => {};
    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $current_key:expr, convert: $conv:tt, key = $new_key:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@req_query_parse $query, $field, $value, key: $new_key, convert: $conv $(, $($rest)*)?)
    };
    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $current_key:expr, convert: $conv:tt, convert = $new_conv:ident $(, $($rest:tt)*)?) => {
        define_request_builder!(@req_query_parse $query, $field, $value, key: $current_key, convert: $new_conv $(, $($rest)*)?)
    };

    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $current_key:expr, convert: $conv:tt, into $(, $($rest:tt)*)?) => {
        define_request_builder!(@req_query_parse $query, $field, $value, key: $current_key, convert: $conv $(, $($rest)*)?)
    };
    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $current_key:expr, convert: $conv:tt, method = $method:tt $(, $($rest:tt)*)?) => {
        define_request_builder!(@req_query_parse $query, $field, $value, key: $current_key, convert: $conv $(, $($rest)*)?)
    };

    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $key:expr, convert: $conv:tt $(,)?) => {
        define_request_builder!(@convert $query, $key, $value, $conv)
    };
    (@req_query_parse $query:expr, $field:ident, $value:expr, key: $key:expr, convert: none $(,)?) => {
        $query.append_pair($key, $value)
    };

    (@opt_query $query:expr, $field:ident, $value:expr, [$($config:tt)*]) => {
        if let Some(val) = $value {
            define_request_builder!(@req_query_parse $query, $field, val, key: stringify!($field), convert: none, $($config)*);
        }
    };
    (@opt_query $query:expr, $field:ident, $value:expr) => {
        if let Some(val) = $value {
            $query.append_pair(stringify!($field), val);
        }
    };

    (@convert $query:expr, $key:expr, $value:expr, none) => {
        $query.append_pair($key, $value);
    };
    (@convert $url:expr, $key:expr, $value:expr, as_ref) => {
        $url.append_pair($key, $value.as_ref());
    };
    (@convert $url:expr, $key:expr, $value:expr, to_string) => {
        $url.append_pair($key, &$value.to_string());
    };
    (@convert $url:expr, $key:expr, $value:expr, rfc3339) => {
        $url.append_pair($key, &$value.to_rfc3339());
    };
    (@convert $url:expr, $key:expr, $value:expr, rfc3339_opt) => {
        $url.append_pair($key, &$value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    };
    (@convert $url:expr, $key:expr, $value:expr, bool) => {
        $url.append_pair($key, &$value.to_string());
    };
    (@convert $url:expr, $key:expr, $value:expr, extend) => {
        for item in $value.iter() {
            $url.append_pair($key, item);
        }
    };
    (@convert $url:expr, $key:expr, $value:expr, extend_as_ref) => {
        for item in $value.iter() {
            $url.append_pair($key, item.as_ref());
        }
    };
    (@convert $url:expr, $key:expr, $value:expr, timezone) => {
        $url.append_pair($key, $value.name());
    };

    (@headers $self:ident, json) => {
        $self.api.header_json()
    };
    (@headers $self:ident, ) => {
        $self.api.default_headers()
    };
    (@headers $self:ident, jwt, $token:ident) => {
        $self.api.build_jwt_headers(&$self.$token)
    };

    (@body $body:expr) => { $body };
    (@body) => { None }
}

/// # Query
/// - `skip` - skip query
/// - `key = CONSTANT` - custom query parameter name
/// - `convert = type` - conversion startegy
///     - as_ref
///     - to_string
///     - extend
#[cfg(any(
    feature = "ads",
    feature = "ccls",
    feature = "channel-points",
    feature = "charity",
    feature = "clips",
    feature = "extensions",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "subscriptions",
    feature = "teams",
    feature = "whisper",
))]
macro_rules! simple_endpoint {
    (
        fn $name:ident(
            $($param:ident: $type:ty $([$($config:tt)*])?),* $(,)?
        ) -> $return:ty;
        endpoint: $endpint:ident,
        method: $method:ident,
        path:[$($path:tt)*]
        $(, headers: [$($header:tt)*])?
        $(, body: {$($body:tt)*})?
        $(,)?
) => {
    fn $name(&self, $($param: $type),*) -> $crate::request::TwitchAPIRequest<$return> {
        let mut url = self.build_url();
        url.path_segments_mut().unwrap().extend(&[$($path)*]);

        {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            let mut query = url.query_pairs_mut();
            $(
                simple_endpoint!(@query query, $param, $param $(, [$($config)*])?);
            )*
        }

        let headers = simple_endpoint!(@headers self $([$($header)*])?);
        let body = simple_endpoint!(@body $({$($body)*})?);
        $crate::request::TwitchAPIRequest::new(
            crate::request::EndpointType::$endpint,
            url,
            reqwest::Method::$method,
            headers,
            body,
            self.client.clone()
        )
    }
    };
    (@body) => { None };
    (@body {$($body:tt)*}) => { {$($body)*} };

    (@headers $self:ident) => { $self.default_headers() };
    (@headers $self:ident [json]) => { $self.header_json() };
    (@headers $self:ident [jwt, $token:expr]) => { $self.build_jwt_headers(&$token) };

    (@query $query:expr, $param:ident, $value:expr, [$($config:tt)*]) => {
        simple_endpoint!(@query_parse $query, $param, $value, key: stringify!($param), $($config)*)
    };
    (@query $query:expr, $param:ident, $value:expr) => {
        $query.append_pair(stringify!($param), $value);
    };

    // Skip
    (@query_parse $query:expr, $param:ident, $value:expr, key: $current_key:expr, skip $(, $($rest:tt)*)?) => {};

    // optinal query
    (@query_parse $query:expr, $param:ident, $value:expr, key: $key:expr, opt, key = $new_key:tt, convert = $conv:tt $(, $($rest:tt)*)?) => {
        if let Some(value) = $value {
            simple_endpoint!(@convert $query, $new_key, value, $conv);
        }
    };
    (@query_parse $query:expr, $param:ident, $value:expr, key: $key:expr, opt $(, $($rest:tt)*)?) => {
        if let Some(value) = $value {
            simple_endpoint!(@query_parse $query, $param, value, key: $key $(, $($rest)*)?);
        }
    };

    (@query_parse $query:expr, $param:ident, $value:expr, key: $current_key:expr, key = $new_key:tt $(, $($rest:tt)*)?) => {
        simple_endpoint!(@query_parse $query, $param, $value, key: $new_key $(, $($rest)*)?)
    };
    (@query_parse $query:expr, $param:ident, $value:expr, key: $key:expr, convert = $conv:tt $(, $($rest:tt)*)?) => {
        simple_endpoint!(@query_parse $query, $param, $value, key: $key, convert: $conv $(, $($rest)*)?)
    };
    (@query_parse $query:expr, $param:ident, $value:expr, key: $key:expr, convert: $conv:tt $(,)?) => {
        simple_endpoint!(@convert $query, $key, $value, $conv)
    };
    (@query_parse $query:expr, $param:ident, $value:expr, key: $key:expr $(,)?) => {
        $query.append_pair($key, $value)
    };

    (@convert $query:expr, $key:expr, $value:expr) => {
        $query.append_pair($key, $value)
    };
    (@convert $query:expr, $key:expr, $value:expr, as_ref) => {
        $query.append_pair($key, $value.as_ref())
    };
    (@convert $query:expr, $key:expr, $value:expr, to_string) => {
        $query.append_pair($key, &$value.to_string())
    };
    (@convert $query:expr, $key:expr, $value:expr, extend) => {
        $query.extend_pairs($value.iter().map(|item| ($key, item)))
    };
    (@convert $query:expr, $key:expr, $value:expr, extend_as_ref) => {
        $query.extend_pairs($value.iter().map(|item| ($key, item.as_ref())))
    };
}

#[cfg(any(
    feature = "extensions",
    feature = "moderation",
    feature = "schedule",
    feature = "users"
))]
macro_rules! opt_method {
    ($name:ident, &$lt:lifetime $ty:ty) => {
        pub fn $name(mut self, value: &$lt $ty) -> Self {
            self.$name = Some(value);
            self
        }
    };
    ($name:ident, $ty:ty [$($config:tt)*]) => {
        opt_method!(@interal $name, $ty, $($config)*);
    };
    ($name:ident, $ty:ty) => {
        pub fn $name(mut self, value: $ty) -> Self {
            self.$name = Some(value);
            self
        }
    };

    (@interal $name:ident, $ty:ty, into $(, $($config:tt)*)?) => {
        pub fn $name(mut self, value: impl Into<$ty>) -> Self {
            self.$name = Some(value.into());
            self
        }
    };
    (@interal $name:ident, $ty:ty, timezone $(, $($config:tt)*)?) => {
        pub fn $name(mut self, value: $ty) -> Self {
            self.$name = Some(value.name());
            self
        }
    };
    (@interal $name:ident, $ty:ty, rfc3339_opt $(, $($config:tt)*)?) => {
        pub fn $name(mut self, value: $ty) -> Self {
            self.$name = Some(&value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
            self
        }
    };
}
