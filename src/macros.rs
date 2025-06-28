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

macro_rules! apply_url {
    (@req $url:expr, $key:expr, $value:expr, $conv:tt) => {
        apply_url!(@convert $url, $key, $value, $conv);
    };

    (@req $url:expr, $key:expr, $value:expr) => {
        $url.query($key, $value);
    };

    (@opt $url:expr, $key:expr, $value:expr, $conv:tt) => {
        if let Some(val) = $value {
            apply_url!(@convert $url, $key, val, $conv);
        }
    };

    (@opt $url:expr, $key:expr, $value:expr) => {
        if let Some(val) = $value {
            $url.query($key, val);
        }
    };

    (@convert $url:expr, $key:expr, $value:expr, u64) => {
        $url.query_u64($key, $value);
    };

    (@convert $url:expr, $key:expr, $value:expr, bool) => {
        $url.query($key, $value.to_string());
    };

    (@convert $url:expr, $key:expr, $value:expr, date) => {
        $url.date($key, $value);
    };

    (@convert $url:expr, $key:expr, $value:expr, vec) => {
        for item in $value {
            $url.query($key, item);
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
        apply_to_url $(, $rest:ident)*
    ) => {
        impl$(<$life>)? $name$(<$life>)? {
            pub fn apply_to_url(self, url: &mut crate::base::UrlBuilder) {
                $(
                    apply_url!(@req url, field_type!($req_field $(, $req_key)?), self.$req_field $(, $req_conv)?);
                )*
                $(
                    apply_url!(@opt url, field_type!($opt_field $(, $opt_key)?), self.$opt_field $(, $opt_conv)?);
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
        to_json $(, $rest:ident)*
    ) => {

        impl$(<$life>)? $name$(<$life>)? {
            pub fn to_json(&self) -> Option<String> {
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
