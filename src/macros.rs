#[cfg(any(
    feature = "ads",
    feature = "analytics",
    feature = "bits",
    feature = "channels",
    feature = "channel-points",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    //feature = "eventsub",
    feature = "extensions",
    feature = "games",
    feature = "guest-star",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "streams",
    feature = "users",
    feature = "videos",
    feature = "whispers",
))]
macro_rules! request_struct {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field:ident: $type:ty
        ),*$(,)?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
        $(
            $(#[$field_meta])*
            $field_vis $field: Option<$type>
        ),*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                request_struct!(@opt any $field: $type);
            )*
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
            required {
                $(
                    $(#[$req_meta:meta])*
                    $fv:vis $req_field:ident: $req_type:ty
                ),* $(,)?
            }
        $(,
            optional {
                $(
                    $(#[$opt_meta:meta])*
                    $ofv:vis $opt_field:ident: $opt_type:ty
                ),*$(,)?
            }
        )?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug)]
        pub struct $struct_name {
            $(
                $(#[$req_meta])*
                $fv $req_field: $req_type,
            )*
            $(
                $(
                    $(#[$opt_meta])*
                    $ofv $opt_field: Option<$opt_type>,
                )*
            )?
        }

        impl $struct_name {
            pub fn new(
                $(
                    $req_field: $req_type,
                )*
            ) -> Self {
                Self {
                    $(
                        $req_field,
                    )*
                    $(
                        $(
                            $opt_field: None,
                        )*
                    )?
                }
            }

            $(
                $(
                    request_struct!(@opt any $opt_field: $opt_type);
                )*
            )?
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
        $(
            string {
                $(
                    $(#[$string_meta:meta])*
                    $string_vis:vis $string_field:ident: $string_type:ty
                ),* $(,)?
            }
        )?
        $(,
            any {
                $(
                    $(#[$any_meta:meta])*
                    $any_vis:vis $any_field:ident: $any_type:ty
                ),* $(,)?
            }
        )?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
        $(
            $(
                $(#[$string_meta])*
                $string_vis $string_field: Option<$string_type>,
            )*
        )?
        $(
            $(
                $(#[$any_meta])*
                $any_vis $any_field: Option<$any_type>,
            )*
        )?
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

        $(
            $(
                request_struct!(@opt string $string_field);
            )*
        )?

        $(
            $(
                request_struct!(@opt any $any_field: $any_type);
            )*
        )?
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
    (@into_request_body $struct_name:ident $into_body:expr)=>{
        impl crate::request::IntoRequestBody for $struct_name {
            fn as_body(&self) -> Option<String> {
                Some(serde_json::to_string(self).unwrap())
            }
        }
    };
    (@opt any $field:ident: $type:ty) => {
        pub fn $field(mut self, value: $type) -> Self {
            self.$field = Some(value);
            self
        }
    };
    (@opt string $field:ident) => {
        pub fn $field<T: Into<String>>(mut self, value: T) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
}

macro_rules! define_request_query {
    (
        $(#[$struct_meta:meta])*
        $name:ident$(<$life:lifetime>)? {
            $(req: {
                $(
                    $(#[$req_meta:meta])*
                    $req_field:ident: $req_type:ty $(=> $req_key:tt)? $(as $req_conv:tt)?
                ),* $(,)?
            })? $(,)?
            $(opts: {
                $(
                    $(#[$opt_meta:meta])*
                    $opt_field:ident: $opt_type:ty $(=> $opt_key:tt)? $(as $opt_conv:tt)?
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
            pub fn new($($($req_field: $req_type),*)?) -> Self {
                Self {
                    $($($req_field,)*)?
                    $($($opt_field: None,)*)?
                }
            }

            $($(
                pub fn $opt_field(mut self, value: $opt_type) -> Self {
                    self.$opt_field = Some(value);
                    self
                }
            )*)?

            #[inline]
            pub fn apply_to_url(self, url: &mut crate::base::UrlBuilder) {
                $($(
                    define_request_query!(@apply_req url,
                        define_request_query!(@field_type $req_field $(, $req_key)?),
                        self.$req_field $(, $req_conv)?
                    );
                )*)?
                $($(
                    define_request_query!(@apply_opt url,
                        define_request_query!(@field_type $opt_field $(, $opt_key)?),
                        self.$opt_field $(, $opt_conv)?
                    );
                )*)?
            }
        }

        define_request_query!(@check_flags $name $(<$life>)? ; $($($flags),*)?);
    };

    (@field_type $field:ident, $const:ident) => { crate::types::constants::$const };
    (@field_type $field:ident, $key:literal) => { $key };
    (@field_type $field:ident) => { stringify!($field) };
    (@field_type $other:ty) => { $other };

    (@apply_req $url:expr, $key:expr, $value:expr) => {
        $url.query($key, $value);
    };
    (@apply_req $url:expr, $key:expr, $value:expr, $conv:tt) => {
        define_request_query!(@convert $url, $key, val, $conv);
    };

    (@apply_opt $url:expr, $key:expr, $value:expr, $conv:tt) => {
        if let Some(val) = $value {
            define_request_query!(@convert $url, $key, val, $conv);
        }
    };
    (@apply_opt $url:expr, $key:expr, $value:expr) => {
        if let Some(val) = $value {
            $url.query($key, val);
        }
    };

    (@convert $url:expr, $key:expr, $value:expr, u64) => {
        $url.query_u64($key, $value);
    };
    (@convert $url:expr, $key:expr, $value:expr, { $conv:expr }) => {
        let converted = $conv($value);
        $url.query($key, converted);
    };
    (@convert $url:expr, $key:expr, $value:expr, $func:ident) => {
            let converted = $func($value);
            $url.query($key, converted);
    };

    (@check_flags $name:ident $(<$life:lifetime>)? ; $($flags:ident),+) => {
        define_request_query!(@apply_flags $name $(<$life>)? ; $($flags),*);
    };
    (@check_flags $name:ident $(<$life:lifetime>)? ; ) => {};

    (@apply_flags $name:ident $(<$life:lifetime>)? ; into_request_body $(, $rest:ident)*) => {
        impl$(<$life>)? crate::request::IntoRequestBody for $name$(<$life>)? {
            fn as_body(&self) -> Option<String> {
                Some(serde_json::to_string(self).unwrap())
            }
        }

        define_request_query!(@apply_flags $name $(<$life>)? ; $($rest),*);
    };
    (@apply_flags $name:ident $(<$life:lifetime>)? ; ) => {};
}
