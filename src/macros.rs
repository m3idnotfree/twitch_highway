macro_rules! field_type {
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

macro_rules! into_query {
    (@req $url:expr, $key:expr, $value:expr, $conv:tt) => {
        apply_url!(@convert $url, $key, $value, $conv);
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

macro_rules! define_request {
    (
        $(#[$struct_meta:meta])*
        $name:ident$(<$life:lifetime>)? {
            $(req: {
                $(
                    $(#[$req_meta:meta])*
                    $req_field:ident: $req_type:ty
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
            pub fn new($($( $req_field: $req_type),* )?) -> Self {
                Self {
                    $($( $req_field, )*)?
                    $($( $opt_field: None, )*)?
                }
            }

            $($(
                pub fn $opt_field(mut self, value: $opt_type) -> Self {
                    self.$opt_field = Some(value);
                    self
                }
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
}

macro_rules! define_select {
    (
        $(#[$struct_meta:meta])*
        $name:ident$(<$life:lifetime>)? {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $field_type:ty
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
                define_select!(@method $field, $field_type, $($method_name)?);
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

    (@method $field:ident, $field_type:ty, $method_name:ident) => {
        pub fn $method_name(value: $field_type) -> Self {
            Self {
                $field: Some(value),
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

/// ### Supported Flags:
/// - `into_query` - Implements `into_query()` method for URL parameter generation
/// - `into_json` - Implements `into_json()` method for JSON serialization
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
                    into_query!(@req url, field_type!($req_field $(, $req_key)?), self.$req_field $(, $req_conv)?);
                )*
                $(
                    into_query!(@opt url, field_type!($opt_field $(, $opt_key)?), self.$opt_field $(, $opt_conv)?);
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
            pub fn into_json(&self) -> Option<String> {
                Some(serde_json::to_string(self).unwrap())
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

/// ### Structure:
/// ```rust,ignore
/// endpoints! {
///     Name {
///         /// Method documentation
///         fn method_name(&self, param) -> ReturnType {
///             endpoint_type: EndpointType,
///             method: HttpMethod,
///             path: ["segment1", "segment2"],
///             ?( query_params: { /* query configuration */}, )
///             ?( headers: [hedaers_config], )
///             ?( body: body_expression )
///         }
///     }
/// }
/// ```
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
/// - (empty) - Default headers only
///
/// Returns `TwitchAPIRequest<ReturnType>`
///
macro_rules! endpoints {
    (
      $(#[$trait_attr:meta])*
        $trait_name:ident {
            $(
                $(#[$method_attr:meta])*
                fn $method_name:ident(
                    &self
                    $(, $param_name:ident: $param_type:ty)* $(,)?
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
        $(#[$trait_attr])*
        pub trait $trait_name {
            $(
                $(#[$method_attr])*
                fn $method_name(
                    &self,
                    $($param_name: $param_type),*
                ) -> TwitchAPIRequest<$return_type>;
            )+
        }

        impl $trait_name for TwitchAPI {
            $(
                fn $method_name(
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
        mod __base_tests {
            #![allow(unused_imports)]
            use crate::test_utils::{params, TwitchApiTest};
            use crate::types::{self, constants::*};

            use super::$trait_name;

            $(
                #[tokio::test]
                pub(crate) async fn $method_name() {
                    let suite = TwitchApiTest::new().await;

                    suite.$method_name().await;

                    let path = format!("/{}", [$($path_segment),*].join("/"));

                    let response = suite
                        .execute(&path, |api| {
                            let params = params::$trait_name::$method_name();
                            api.$method_name($(params.$param_name),*)
                        })
                        .json()
                        .await;

                    assert!(response.is_ok());
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
