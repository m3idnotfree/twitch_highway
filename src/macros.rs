/// Resolves field names to their corresponding query parameter keys.
///
/// # Patterns:
/// - `field_key!(field_name, CONSTANT)` → `crate::types::constants::CONSTANT`
/// - `field_key!(field_name, "literal")` → `"literal"`  
/// - `field_key!(field_name)` → `"field_name"` (stringified)
/// - `field_key!(other_type)` → `other_type` (pass-through)
///
/// # Example:
/// ```rust,ignore
/// field_key!(user_id, USER_ID)     // → crate::types::constants::USER_ID
/// field_key!(custom, "custom_key") // → "custom_key"
/// field_key!(name)                 // → "name"
/// ```
macro_rules! field_key {
    ($field:ident, $const:ident) => {
        crate::types::constants::$const
    };

    ($field:ident, $key:literal) => {
        $key
    };

    ($field:ident) => {
        stringify!($field)
    };

    ($other:ty) => {
        $other
    };
}

/// Converts Rust values to URL query parameters with type-specific formatting.
///
/// # Patterns:
/// - `@req` - Required parameter (always added)
/// - `@opt` - Optional parameter (added only if Some)
/// - `@convert` - Type-specific conversion logic
///
/// # Supported Conversions:
/// - `u64` - Efficient integer formatting using itoa
/// - `bool` - String conversion via to_string()
/// - `date` - RFC3339 timestamp formatting
/// - `vec` - Multiple parameters with same key
/// - `{expr}` - Custom conversion expression
/// - `func_name` - Custom conversion function
///
/// # Example:
/// ```rust,ignore
/// into_query!(@req url, "user_id", user_id);           // Simple string
/// into_query!(@opt url, "count", Some(50), u64);       // Optional number
/// into_query!(@req url, "created", timestamp, date);   // Date formatting
/// ```
macro_rules! into_query {
    (@req $url:expr, $key:expr, $value:expr, $conv:tt) => {
        into_query!(@convert $url, $key, $value, $conv);
    };

    (@req $url:expr, $key:expr, $value:expr) => {
        $url.query($key, $value);
    };

    (@opt $url:expr, $key:expr, $value:expr, $conv:tt) => {
        if let Some(val) = $value {
            into_query!(@convert $url, $key, val, $conv);
        }
    };

    (@opt $url:expr, $key:expr, $value:expr) => {
        if let Some(val) = $value {
            $url.append_pair($key, val.as_ref());
        }
    };

    (@convert $url:expr, $key:expr, $value:expr, u64) => {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format($value);

        $url.append_pair($key, s);
    };

    (@convert $url:expr, $key:expr, $value:expr, bool) => {
        $url.append_pair($key, &$value.to_string());
    };

    (@convert $url:expr, $key:expr, $value:expr, date) => {
        $url.append_pair($key, &$value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    };

    (@convert $url:expr, $key:expr, $value:expr, vec) => {
        for item in $value {
            $url.append_pair($key, item.as_ref());
        }
    };

    (@convert $url:expr, $key:expr, $value:expr, { $conv:expr }) => {
        let converted = $conv($value);
        $url.query($key, converted);
    };

    (@convert $url:expr, $key:expr, $value:expr, $func:ident) => {
            let converted = $func($value);
            $url.query($key, converted);
    };
}


/// Generates selection structs where all fields are optional with static factory methods.
///
/// # Structure:
/// ```rust,ignore
/// define_select! {
///     #[derive(Debug)]
///     SelectName {
///         field1: Type => "key" ; conversion,
///         field2: Type = default_value,
///         field3: Type as custom_method,
///     };
///     flags
/// }
/// ```
///
/// # Features:
/// - All fields are `Option<T>`
/// - Implements `Default` trait
/// - Generates static factory methods for each field
/// - Custom method names via `as method_name`
/// - Default values via `= expression`
///
/// # Example:
/// ```rust,ignore
/// define_select! {
///     UserSelect {
///         name: String,
///         age: u32 = 18,
///         active: bool as is_active,
///     };
///     into_query
/// }
///
/// // Usage:
/// let select = UserSelect::name("alice".to_string());
/// let select = UserSelect::is_active(true);
/// ```
#[cfg(any(
    feature = "clips",
    feature = "streams",
    feature = "teams",
    feature = "videos",
))]
macro_rules! define_select {
    (
        $(#[$struct_meta:meta])*
        $name:ident$(<$life:lifetime>)? {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $field_type:ty
                $(| $into:tt $(: $item_ty:ty)? )?
                $(=> $key:tt)?
                $(as $method_name:ident)?
                $(= $default:expr)?
                $(; $conv:tt)?
            ),* $(,)?
        }
        $(; $($flags:ident),*)?
    ) => {
        $(#[$struct_meta])*
        pub struct $name$(<$life>)? {
            $(
                $(#[$field_meta])*
                pub $field: Option<$field_type>,
            )*
        }

        impl$(<$life>)? Default for $name$(<$life>)? {
            fn default() -> Self {
                Self {
                    $(
                        $field: define_select!(@default_value $($default)?),
                    )*
                }
            }
        }

        impl$(<$life>)? $name$(<$life>)? {
            #[inline]
            pub fn new() -> Self {
                Self::default()
            }
            $(
                define_select!(@method $field, $field_type, $($method_name)? $(| $into $(: $item_ty)?)?);
            )*
        }

        automatic_impl!(
            $name$(<$life>)? ;
            req: [] ;
            opts: [$($field : $field_type $(=> $key)? $(; $conv)?),*] ;
            $($( $flags ),*)?
        );
    };

    (@default_value $default:expr) => { Some($default) };
    (@default_value) => { None };

    (@method $field:ident, $field_type:ty, $method_name:ident | into) => {
        pub fn $method_name(value: impl Into<$field_type>) -> Self {
            Self {
                $field: Some(value.into()),
                ..Default::default()
            }
        }
    };

    (@method $field:ident, $field_type:ty, $method_name:ident) => {
        pub fn $method_name(value: $field_type) -> Self {
            Self {
                $field: Some(value),
                ..Default::default()
            }
        }
    };

    (@method $field:ident, $field_type:ty, | into) => {
        pub fn $field(value: impl Into<$field_type>) -> Self {
            Self {
                $field: Some(value.into()),
                ..Default::default()
            }
        }
    };

    (@method $field:ident, $field_type:ty,) => {
        pub fn $field(value: $field_type) -> Self {
            Self {
                $field: Some(value),
                ..Default::default()
            }
        }
    };
}

/// Automatically implements traits based on specified flags.
///
/// # Supported Flags:
/// - `into_query` - Implements `into_query()` method for URL parameter generation
/// - `into_json` - Implements `into_json()` method for JSON serialization
///
/// # Usage:
/// This macro is typically called from `define_request!` or `define_select!`
/// and should not be used directly.
///
/// # Generated Methods:
/// - `into_query(url_serializer)` - Converts struct to URL query parameters
/// - `into_json() -> Option<String>` - Converts struct to JSON string
macro_rules! automatic_impl {
    (
        $name:ident$(<$life:lifetime>)? ;
        req: [$($req_field:ident : $req_field_type:ty $(=> $req_key:tt)? $(; $req_conv:tt)?),*] ;
        opts: [$($opt_field:ident : $opt_field_type:ty $(=> $opt_key:tt)? $(; $opt_conv:tt)?),*] ;
        $($flags:ident),*
    ) => {
        automatic_impl!(@impl
            $name$(<$life>)? ;
            req: [$($req_field: $req_field_type $(=> $req_key)? $(; $req_conv)?),*] ;
            opts: [$($opt_field: $opt_field_type $(=> $opt_key)? $(; $opt_conv)?),*] ;
            $($flags),*
        );
    };

    (@impl $name:ident$(<$life:lifetime>)? ;
        req: [$($req_field:ident : $req_field_type:ty $(=> $req_key:tt)? $(; $req_conv:tt)?),*] ;
        opts: [$($opt_field:ident : $opt_field_type:ty $(=> $opt_key:tt)? $(; $opt_conv:tt)?),*] ;
        into_query $(, $rest:ident)*
    ) => {
        impl$(<$life>)? $name$(<$life>)? {
            pub fn into_query(self, url: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>>) {
                $(
                    into_query!(@req url, field_key!($req_field $(, $req_key)?), self.$req_field $(, $req_conv)?);
                )*
                $(
                    into_query!(@opt url, field_key!($opt_field $(, $opt_key)?), self.$opt_field $(, $opt_conv)?);
                )*
            }
        }

        automatic_impl!(@impl $name$(<$life>)? ;
            req: [$($req_field: $req_field_type $(=> $req_key)? $(; $req_conv)?),*] ;
            opts: [$($opt_field: $opt_field_type $(=> $opt_key)? $(; $opt_conv)?),*] ;
            $($rest),*
        );
    };

    (@impl $name:ident$(<$life:lifetime>)? ;
        req: [$($req_field:ident : $req_field_type:ty $(=> $req_key:tt)? $(; $req_conv:tt)?),*] ;
        opts: [$($opt_field:ident : $opt_field_type:ty $(=> $opt_key:tt)? $(; $opt_conv:tt)?),*] ;
        into_json $(, $rest:ident)*
    ) => {

        impl$(<$life>)? $name$(<$life>)? {
            pub fn into_json(self) -> Option<String> {
                Some(serde_json::to_string(&self).unwrap())
            }
        }

        automatic_impl!(@impl $name$(<$life>)? ;
            req: [$($req_field: $req_field_type $(=> $req_key)? $(; $req_conv)?),*] ;
            opts: [$($opt_field: $opt_field_type $(=> $opt_key)? $(; $opt_conv)?),*] ;
            $($rest),*

        );
    };

    (@impl $name:ident$(<$life:lifetime>)? ;
        req: [$($req_field:ident : $req_field_type:ty $(=> $req_key:tt)? $(; $req_conv:tt)?),*] ;
        opts: [$($opt_field:ident : $opt_field_type:ty $(=> $opt_key:tt)? $(; $opt_conv:tt)?),*] ;
    ) => {}
}

/// Generates API endpoint traits with implementations.
///
/// ### Structure:
/// ```rust,ignore
/// endpoints! {
///     TraitName {
///         /// Documentation for the endpoint
///         fn method_name(&self, param: Type) -> ResponseType {
///             endpoint_type: MethodName,
///             method: GET,
///             path: ["segment1", "segment2"],
///             query_params: {
///                 query("key", value),
///                 opt("optional_key", optional_value),
///                 pagination(pagination_obj),
///                 into_query(custom_params),
///             },
///             headers: [json] | [jwt, token] | [],
///             body: body_expression,
///         }
///     }
/// }
/// ```
///
/// # Generated Components:
/// 1. **Public trait** - Defines the API interface
/// 2. **Implementation** - Implements trait for TwitchAPI
/// 3. **Parameter structs** - For test data (`__params` module)
///
/// ### Query Parameter Patterns:
/// - `query("key", value)` - Add single parameter
/// - `opt("key", optional_value)` - Add if Some
/// - `extend(iterator)` - Add multiple key-value pairs
/// - `opt_extend(optional_iterator)` - Add multiple if Some
/// - `pagination(pagination_obj)` - Add pagination parameters
/// - `into_query(query_builder)` - Use custom query builder
/// - `opt_into_query(optional_builder)` - Use custom builder if Some
///
/// ### Header Patterns:
/// - `json` - Content-Type: application/json
/// - `jwt, token` - JWT authorization with token
/// - `[]` or omitted - Default headers only
///
/// Returns `TwitchAPIRequest<ReturnType>`
macro_rules! endpoints {
    (
      $(#[$trait_attr:meta])*
        $trait_name:ident {
            $(
                $(#[$method_attr:meta])*
                fn $method_name:ident$(<$($life:lifetime),* $($generics:ident $(: $bounds:tt)?),*>)?(
                    &self
                    $(, $param_name:ident: $param_type:ty )* $(,)?
                ) -> $return_type:ty
                {
                        endpoint_type: $endpoint_type:ident,
                        method: $http_method:ident,
                        path: [$($path_segment:expr),* $(,)?]
                        $(, query_params: {$($query_config:tt)*})?
                        $(, headers: [$($header_config:tt)*])?
                        $(, body: $body_expr:expr)?
                        $(,)?
                }
            )+
        }
    ) => {
        #[allow(non_camel_case_types)]
        $(#[$trait_attr])*
        pub trait $trait_name {
            $(
                $(#[$method_attr])*
                fn $method_name$(<$($life),* $($generics $(: $bounds)?),*>)?(
                    &self,
                    $($param_name: $param_type),*
                ) -> crate::request::TwitchAPIRequest<$return_type>;
            )+
        }

        impl $trait_name for crate::TwitchAPI {
            $(
                fn $method_name$(<$($life),* $($generics $(: $bounds)?),*>)?(
                    &self
                    $(, $param_name: $param_type)*
                ) -> crate::request::TwitchAPIRequest<$return_type>
                {
                    let mut url = self.build_url();

                    url.path_segments_mut().unwrap().extend([$($path_segment),*]);

                    $( endpoints!(@query url.query_pairs_mut(), $($query_config)*); )?

                    let headers = endpoints!(@headers self, $($($header_config)*)?);
                    let body = endpoints!(@body_handler $($body_expr)?);

                    crate::request::TwitchAPIRequest::new(
                        crate::request::EndpointType::$endpoint_type,
                        url,
                        reqwest::Method::$http_method,
                        headers,
                        body,
                        self.client.clone()
                    )
                }
            )+
        }
    };

    // Query parameter handlers

    // Add a single query parameter
    (@query $url:expr, query($key:expr, $value:expr) $(, $($rest:tt)*)?) => {
        $url.append_pair($key, $value.as_ref());
        $(
            endpoints!(@query $url, $($rest)*);
        )?
    };

    // Add optional query parameter (only if Some)
    (@query $url:expr, opt($key:expr, $value:expr) $(, $($rest:tt)*)?) => {
        if let Some(val) = $value {
            $url.append_pair($key, val.as_ref());
        }
        $(
            endpoints!(@query $url, $($rest)*);
        )?
    };

    // Extend query parameters from iterator
    (@query $url:expr, extend($iter:expr) $(, $($rest:tt)*)?) => {
        $url.extend_pairs($iter);
        $(
            endpoints!(@query $url, $($rest)*);
        )?

    };

    // Extend query parameters from optional iterator
    (@query $url:expr, opt_extend($iter:expr) $(, $($rest:tt)*)?) => {
        if let Some(opts) = $iter {
            $url.extend_pairs(opts);
        }
        $(
            endpoints!(@query $url, $($rest)*);
        )?
    };

    // Add pagination query parameters
    (@query $url:expr, pagination($pagination:expr) $(, $($rest:tt)*)?) => {
        if let Some(pagination) = $pagination {
            pagination.into_query(&mut $url);
        }
        $(
            endpoints!(@query $url, $($rest)*);
        )?
    };

    // Use custom query parameter builder
    (@query $url:expr, into_query($custom_applier:expr) $(, $($rest:tt)*)?) => {
        $custom_applier.into_query(&mut $url);
        $(
            endpoints!(@query $url, $($rest)*);
        )?
    };

    // Use optional custom query parameter builder
    (@query $url:expr, opt_into_query($custom_applier:expr) $(, $($rest:tt)*)?) => {
        if let Some(opts) = $custom_applier {
            opts.into_query(&mut $url);
        }
        $(
            endpoints!(@query $url, $($rest)*);
        )?
    };

    // Base case: no more query parameters
    (@query $url:expr,) => {};

    // Header configuration handlers

    // JSON content type headers
    (@headers $self:ident, json) => {
        $self.header_json()
    };

    // JWT authorization headers
    (@headers $self:ident, jwt, $token:expr) => {
        $self.build_jwt_headers(&$token)
    };

    // Default headers only
    (@headers $self:ident, ) => {
        $self.default_headers()
    };

    // Request body handlers

    // Use provided body expression
    (@body_handler $body_expr:expr) => {
        $body_expr
    };

    // No body (None)
    (@body_handler) => {
        None
    };
}

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
}
