#[cfg(any(
    feature = "ads",
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    //feature = "eventsub",
    feature = "extensions",
    feature = "games",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
    feature = "videos",
    ))]
macro_rules! request_struct {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
        $($(#[$field_meta:meta])*
            $field_vis:vis $field:ident: $type:ty
        ),*$(,)?
        }
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
        $($(#[$field_meta])*
            $field_vis $field: Option<$type>
        ),*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

         $(pub fn $field(mut self, value: $type) -> Self
            {
                self.$field = Some(value);
                self
            })*
        }
    };
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
            required {
            $($(#[$req_meta:meta])*
            $fv:vis $req_field:ident: $req_type:ty ),*$(,)?
            }
        $(, optional {
            $($(#[$opt_meta:meta])*
              $ofv:vis $opt_field:ident: $opt_type:ty ),*$(,)?
            }
        )?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug)]
        pub struct $struct_name {
            $($(#[$req_meta])*
               $fv $req_field: $req_type,
            )*
         $($($(#[$opt_meta])*
               $ofv $opt_field: Option<$opt_type>,
         )*)?
        }

        impl $struct_name {
            pub fn new(
            $($req_field: $req_type,)*
            ) -> Self {
                Self {
                $($req_field,)*
                $($($opt_field: None,)*)?
                }
            }

            $($(
                pub fn $opt_field(mut self, value: $opt_type) -> Self
                {
                    self.$opt_field = Some(value);
                    self
                }
            )*)?
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
        (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
            required {
                $(string {
                    $($(#[$req_str_meta:meta])*
                    $fv_str:vis $req_str_field:ident: $req_str_type:ty ),*$(,)?
                },)?
                $(any {
                    $($(#[$req_any_meta:meta])*
                    $fv_any:vis $req_any_field:ident: $req_any_type:ty ),*$(,)?

                })?
            }
        $(, optional {
                $(string {
                    $($(#[$opt_str_meta:meta])*
                    $fv_opt_str:vis $opt_str_field:ident: $opt_str_type:ty ),*$(,)?
                },)?
                $(any {
                    $($(#[$opt_any_meta:meta])*
                    $fv_opt_any:vis $opt_any_field:ident: $opt_any_type:ty ),*$(,)?

                })?

            }
        )?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug)]
        pub struct $struct_name {
            $($(#[$req_str_meta])*
               $fv_str $req_str_field: $req_str_type,
            )*
            $($(#[$req_any_meta])*
               $fv_any $req_any_field: $req_any_type,
            )*
         $($($(#[$opt_str_meta])*
               $fv_opt_str $opt_str_field: Option<$opt_str_type>,
         )*)?
         $($($(#[$opt_any_meta])*
               $fv_opt_any $opt_any_field: Option<$opt_any_type>,
         )*)?
        }

        impl $struct_name {
            pub fn new(
            $($req_str_field: impl Into<String>,)*
            $($req_any_field: $req_any_type,)*
            ) -> Self {
                Self {
                $($req_field,)*
                $($req_any_field,)*
                $($($opt_str_field: None,)*)?
                $($($opt_any_field: None,)*)?
                }
            }

            $($(
                pub fn $opt_str_field(mut self, value: $opt_str_type) -> Self
                {
                    self.$opt_str_field = Some(value);
                    self
                }
            )*)?
            $($(
                pub fn $opt_any_field(mut self, value: $opt_any_type) -> Self
                {
                    self.$opt_any_field = Some(value);
                    self
                }
            )*)?
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };

    (@into_request_body $struct_name:ident $into_body:expr)=>{
        impl crate::IntoRequestBody for $struct_name {
            fn as_body(&self) -> Option<String> {
                Some(serde_json::to_string(self).unwrap())
            }
        }
    };
            (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
    $(  string {
            $($(#[$string_meta:meta])*
                $string_vis:vis $string_field:ident: $string_type:ty),*$(,)?
        }
    )?
    $(,
        any {
            $($(#[$any_meta:meta])*
                $any_vis:vis $any_field:ident: $any_type:ty),*$(,)?
        }
    )?
    }
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
            $(
        $($(#[$string_meta])*
            $string_vis $string_field: Option<$string_type>,)*
            )?
        $($($(#[$any_meta])*
            $any_vis $any_field: Option<$any_type>,
        )*)?
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

        $(
         $(pub fn $string_field<T: Into<String>>(mut self, value: T) -> Self
            {
                self.$string_field = Some(value.into());
                self
            }
         )*
        )?

        $(
         $(pub fn $any_field(mut self, value: $any_type) -> Self
            {
                self.$any_field = Some(value);
                self
            }
         )*
        )?
        }
    };

}

#[cfg(any(
    feature = "channel_points",
    feature = "channels",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "schedule",
    feature = "streams",
))]
macro_rules! new_request_struct {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
    $(  string: {
            $($(#[$string_meta:meta])*
                $string_vis:vis $string_field:ident: $string_type:ty),*$(,)?
        }
    )?
    $(,
        any: {
            $($(#[$any_meta:meta])*
                $any_vis:vis $any_field:ident: $any_type:ty),*$(,)?
        }
    )?
    }
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
            $(
        $($(#[$string_meta])*
            $string_vis $string_field: Option<$string_type>,)*
            )?
        $($($(#[$any_meta])*
            $any_vis $any_field: Option<$any_type>,
        )*)?
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

        $(
         $(pub fn $string_field<T: Into<String>>(mut self, value: T) -> Self
            {
                self.$string_field = Some(value.into());
                self
            }
         )*
        )?

        $(
         $(pub fn $any_field(mut self, value: $any_type) -> Self
            {
                self.$any_field = Some(value);
                self
            }
         )*
        )?
        }
    };
}
