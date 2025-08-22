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

/// Generates request structs with builder pattern methods for API parameters.
///
/// # Structure:
/// ```rust,ignore
/// define_request! {
///     #[derive(Debug, Serialize)]
///     RequestName {
///         req: {
///             required_field: Type => "api_key" ; conversion,
///         },
///         opts: {
///             optional_field: Type => CONSTANT ; conversion,
///         };
///         flags  // e.g., into_query, into_json
///     }
/// }
/// ```
///
/// # Field Options:
/// - `| into` - Enable `impl Into<T>` conversion for parameters (accepts &str for String, etc.)
/// - `=> key` - Custom query parameter name (literal or constant)
/// - `as method_name` - Custom builder method name
/// - `= default` - Default value for the field
/// - `; conversion` - Type conversion for URL encoding
///
/// # Special Behavior:
/// - Fields with `| into` flag automatically get `impl Into<T>` parameters
/// - Required fields with `| into` accept conversion types in `new()` constructor
/// - Optional fields with `| into` accept conversion types in builder methods
/// - Other types use exact type matching
/// - Generates `new()` constructor and fluent builder methods
///
/// # Example:
/// ```rust,ignore
/// define_request! {
///     #[derive(Debug)]
///     UserRequest {
///         req: {
///             user_id: UserId => USER_ID,
///         },
///         opts: {
///             name: String | into,
///             count: u64 ; u64,
///         };
///         into_query
///     }
/// }
///
/// // Usage:
/// let req = UserRequest::new(user_id).name("alice").count(10);
/// ```
macro_rules! define_request {
    (
        $(#[$struct_meta:meta])*
        $name:ident$(<$life:lifetime>)? {
            $(req: {
                $(
                    $(#[$req_meta:meta])*
                    $req_field:ident: $req_type:ty
                    $(| $req_flags:tt)?
                    $(=> $req_key:tt)?
                    $(as $req_method_name:ident)?
                    $(= $req_default:expr)?
                    $(; $req_conv:tt)?
                ),* $(,)?
            })? $(,)?
            $(opts: {
                $(
                    $(#[$opt_meta:meta])*
                    $opt_field:ident: $opt_type:ty
                    $(| $opt_flags:tt)?
                    $(=> $opt_key:tt)?
                    $(as $opt_method_name:ident)?
                    $(= $opt_default:expr)?
                    $(; $opt_conv:tt)?
                ),* $(,)?
            })?
            $(; $($flags:ident),* $(,)?)?
        }
    ) => {
        $(#[$struct_meta])*
        pub struct $name$(<$life>)? {
            $($(
                $(#[$req_meta])*
                pub $req_field: $req_type,
            )*)?
            $($(
                $(#[$opt_meta])*
                pub $opt_field: Option<$opt_type>,
            )*)?
        }

        impl$(<$life>)? $name$(<$life>)? {
            #[allow(clippy::new_without_default)]
            pub fn new(
                $($(
                    $req_field: define_request!(@param_type $req_type $(| $req_flags)?)
                ),*)?
            ) -> Self {
                Self {
                    $($(
                        $req_field: define_request!(@param_value $req_field $(| $req_flags)?),
                    )*)?
                    $($(
                        $opt_field: None,
                    )*)?
                }
            }

            $($(
                define_request!(@opt_method $opt_field: $opt_type $(| $opt_flags)?);
            )*)?
        }

        automatic_impl!(
            $name$(<$life>)? ;
            req: [$($(
                $req_field : $req_type $(=> $req_key)? $(; $req_conv)?
            ),*)?] ;
            opts: [$($(
                $opt_field : $opt_type $(=> $opt_key)? $(; $opt_conv)?
            ),*)?] ;
            $($( $flags ),*)?
        );
    };

    (@param_type $type:ty | into) => {
        impl Into<$type>
    };

    (@param_type $type:ty) => {
        $type
    };

    (@param_value $field:ident | into) => {
        $field.into()
    };

    (@param_value $field:ident) => {
        $field
    };


    (@opt_method $opt_field:ident: $opt_type:ty | into) => {
        pub fn $opt_field(mut self, value: impl Into<$opt_type>) -> Self {
            self.$opt_field = Some(value.into());
            self
        }
    };

    (@opt_method $opt_field:ident: $opt_type:ty) => {
        pub fn $opt_field(mut self, value: $opt_type) -> Self {
            self.$opt_field = Some(value);
            self
        }
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
///             endpoint_type: EndpointType::MethodName,
///             method: Method::GET,
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
                fn $method_name:ident$(<$life:lifetime>)?(
                    &self
                    $(, $param_name:ident: $param_type:ty )* $(,)?
                ) -> $return_type:ty {
                        endpoint_type: $endpoint_type:expr,
                        method: $http_method:expr,
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
                fn $method_name$(<$life>)?(
                    &self,
                    $($param_name: $param_type),*
                ) -> TwitchAPIRequest<$return_type>;
            )+
        }

        impl $trait_name for TwitchAPI {
            $(
                fn $method_name$(<$life>)?(
                    &self
                    $(, $param_name: $param_type)*
                ) -> TwitchAPIRequest<$return_type> {
                    let mut url = self.build_url();

                    url.path_segments_mut().unwrap().extend([$($path_segment),*]);

                    $( endpoints!(@query url.query_pairs_mut(), $($query_config)*); )?

                    let headers = endpoints!(@headers self, $($($header_config)*)?);
                    let body = endpoints!(@body_handler $($body_expr)?);

                    TwitchAPIRequest::new(
                        $endpoint_type,
                        url,
                        $http_method,
                        headers,
                        body,
                    )
                }
            )+
        }

        #[cfg(test)]
        mod __test_enforcement {
            const _ENFORCE_ALL_TESTS_EXIST: () = {
                $(
                    let _: fn() = super::tests::$method_name;
                )*
            };
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

/// Generates standardized integration tests for API endpoints.
///
/// # Patterns:
/// - `api_test!(endpoint, [params])` → Test with JSON response validation
/// - `api_test!(send endpoint, [params])` → Test with HTTP response validation only
/// - `api_test!(extra endpoint, test_name [params])` → Extra Test with custom mock setup
///
/// # Requirements:
/// - `TwitchApiTest` must implement `async fn endpoint(&self)` for mock setup
/// - API trait must have corresponding `fn endpoint(&self, params) -> TwitchAPIRequest<T>`
///
/// # Example:
/// ```rust,ignore
/// api_test!(get_users, [UserId::from("123456789")]);                    // JSON response
/// api_test!(send create_reward, [broadcaster_id, reward_request]);     // No content (204)
/// api_test!(get_global_emotes, []);                                    // No parameters
/// api_test!(extra get_analytics, extra_get_analytics, []);             // Custom test name
/// ```
#[cfg(test)]
macro_rules! api_test {
    ($endpoint:ident, $([$($param:expr),* $(,)?])?) => {
        #[tokio::test]
        pub(crate) async fn $endpoint() {
            let suite = crate::test_utils::TwitchApiTest::new().await;

            suite.$endpoint().await;

            let _ = suite
                .execute(|api| {
                    api.$endpoint($($($param),*)?)
                })
                .json()
                .await
                .unwrap();

        }
    };

    (send $endpoint:ident, $([$($param:expr),* $(,)?])?) => {
        #[tokio::test]
        pub(crate) async fn $endpoint() {
            let suite = crate::test_utils::TwitchApiTest::new().await;

            suite.$endpoint().await;

            let _ = suite
                .execute(|api| {
                    api.$endpoint($($($param),*)?)
                })
                .send()
                .await
                .unwrap();

        }
    };

    (extra $endpoint:ident, $test_endpoint:ident, $([$($param:expr),* $(,)?])?) => {
        #[tokio::test]
        async fn $test_endpoint() {
            let suite = crate::test_utils::TwitchApiTest::new().await;

            suite.$test_endpoint().await;

            let _ = suite
                .execute(|api| {
                    api.$endpoint($($($param),*)?)
                })
                .json()
                .await
                .unwrap();

        }
    };
}
